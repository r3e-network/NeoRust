use std::sync::Arc;

use crate::neo_crypto::utils::ToHexString;
use async_trait::async_trait;
use num_bigint::BigInt;
use primitive_types::H160;

// Replace prelude imports with specific types
use crate::{
	neo_builder::{CallFlags, ScriptBuilder},
	neo_clients::{APITrait, JsonRpcProvider, RpcClient},
	neo_contract::{ContractError, NeoIterator},
	neo_types::{
		Bytes, ContractManifest, ContractParameter, InvocationResult, OpCode, ScriptHash, StackItem,
	},
	ScriptHashExtension,
};

// Import transaction types from the correct modules
use crate::neo_builder::{Signer, TransactionBuilder};

#[async_trait]
pub trait SmartContractTrait<'a>: Send + Sync {
	const DEFAULT_ITERATOR_COUNT: usize = 100;
	type P: JsonRpcProvider;

	async fn name(&self) -> String {
		self.get_manifest().await.name.clone().unwrap_or_default()
	}
	fn set_name(&mut self, _name: String) {
		// NNS contracts don't support setting names
		// This is intentionally a no-op as it's not supported
		tracing::warn!("Cannot set name for NNS contract - operation not supported");
	}

	fn script_hash(&self) -> H160;

	fn set_script_hash(&mut self, _script_hash: H160) {
		// NNS contracts don't support setting script hash
		// This is intentionally a no-op as it's not supported
		tracing::warn!("Cannot set script hash for NNS contract - operation not supported");
	}

	fn provider(&self) -> Option<&RpcClient<Self::P>>;

	async fn invoke_function(
		&self,
		function: &str,
		params: Vec<ContractParameter>,
	) -> Result<TransactionBuilder<Self::P>, ContractError> {
		let script = self.build_invoke_function_script(function, params).await?;
		let mut builder = TransactionBuilder::new();
		builder.set_script(Some(script));
		Ok(builder)
	}

	async fn build_invoke_function_script(
		&self,
		function: &str,
		params: Vec<ContractParameter>,
	) -> Result<Bytes, ContractError> {
		if function.is_empty() {
			return Err(ContractError::InvalidNeoName("Function name cannot be empty".to_string()));
		}

		let script = ScriptBuilder::new()
			.contract_call(&self.script_hash(), function, params.as_slice(), Some(CallFlags::None))
			.map_err(|e| {
				ContractError::RuntimeError(format!("Failed to build contract call: {e}"))
			})?
			.to_bytes();

		Ok(script)
	}

	async fn call_function_returning_string(
		&self,
		function: &str,
		params: Vec<ContractParameter>,
	) -> Result<String, ContractError> {
		let output = self.call_invoke_function(function, params, vec![]).await?;
		self.throw_if_fault_state(&output)?;

		let item = output
			.get_first_stack_item()
			.map_err(|e| ContractError::InvalidResponse(e.to_string()))?;
		match item.as_string() {
			Some(s) => Ok(s),
			None => Err(ContractError::UnexpectedReturnType("String".to_string())),
		}
	}

	async fn call_function_returning_int(
		&self,
		function: &str,
		params: Vec<ContractParameter>,
	) -> Result<i64, ContractError> {
		let output = self.call_invoke_function(function, params, vec![]).await?;
		self.throw_if_fault_state(&output)?;

		let item = output
			.get_first_stack_item()
			.map_err(|e| ContractError::InvalidResponse(e.to_string()))?;
		match item.as_int() {
			Some(i) => Ok(i),
			None => Err(ContractError::UnexpectedReturnType("Int".to_string())),
		}
	}

	async fn call_function_returning_bool(
		&self,
		function: &str,
		params: Vec<ContractParameter>,
	) -> Result<bool, ContractError> {
		let output = self.call_invoke_function(function, params, vec![]).await?;
		self.throw_if_fault_state(&output)?;

		let item = output
			.get_first_stack_item()
			.map_err(|e| ContractError::InvalidResponse(e.to_string()))?;
		match item.as_bool() {
			Some(b) => Ok(b),
			None => Err(ContractError::UnexpectedReturnType("Bool".to_string())),
		}
	}

	// Other methods

	async fn call_invoke_function(
		&self,
		function: &str,
		params: Vec<ContractParameter>,
		signers: Vec<Signer>,
	) -> Result<InvocationResult, ContractError> {
		if function.is_empty() {
			return Err(ContractError::InvalidNeoName("Function cannot be empty".to_string()));
		}

		let provider = self.provider().ok_or_else(|| {
			ContractError::ProviderNotSet(
				"Provider is required for contract invocations".to_string(),
			)
		})?;

		provider
			.invoke_function(&self.script_hash(), function.into(), params, Some(signers))
			.await
			.map_err(ContractError::from)
	}

	fn throw_if_fault_state(&self, output: &InvocationResult) -> Result<(), ContractError> {
		if output.has_state_fault() {
			let message =
				output.exception.clone().unwrap_or_else(|| "Invocation faulted".to_string());
			Err(ContractError::InvocationFailed(message))
		} else {
			Ok(())
		}
	}

	// Other methods for different return types
	async fn call_function_returning_script_hash(
		&self,
		function: &str,
		params: Vec<ContractParameter>,
	) -> Result<H160, ContractError> {
		let output = self.call_invoke_function(function, params, vec![]).await?;
		self.throw_if_fault_state(&output)?;

		let item = output
			.get_first_stack_item()
			.map_err(|e| ContractError::InvalidResponse(e.to_string()))?;

		let bytes = item
			.as_bytes()
			.ok_or_else(|| ContractError::UnexpectedReturnType("ByteString".to_string()))?;

		if bytes.len() != 20 {
			return Err(ContractError::InvalidResponse(format!(
				"Expected 20 bytes for ScriptHash, got {}",
				bytes.len()
			)));
		}

		Ok(H160::from_slice(&bytes))
	}

	async fn call_function_returning_iterator<U>(
		&self,
		function: &str,
		params: Vec<ContractParameter>,
		mapper: Arc<dyn Fn(StackItem) -> U + Send + Sync>,
	) -> Result<NeoIterator<U, Self::P>, ContractError>
	where
		U: Send + Sync, // Adding this bound if necessary
	{
		let output = self.call_invoke_function(function, params, vec![]).await?;
		self.throw_if_fault_state(&output)?;

		let session_id = output.session_id.clone().ok_or_else(|| {
			ContractError::InvalidResponse(
				"No session ID returned from iterator invocation".to_string(),
			)
		})?;

		let item = output
			.get_first_stack_item()
			.map_err(|e| ContractError::InvalidResponse(e.to_string()))?;
		let StackItem::InteropInterface { id, interface: _ } = item else {
			return Err(ContractError::UnexpectedReturnType(format!(
				"Expected InteropInterface, got {:?}",
				item
			)));
		};

		let provider = self.provider().ok_or_else(|| {
			ContractError::ProviderNotSet("Provider is required for iterator traversal".to_string())
		})?;

		Ok(NeoIterator::new(session_id, id.clone(), mapper, Some(provider)))
	}

	async fn call_function_and_unwrap_iterator<U>(
		&self,
		function: &str,
		params: Vec<ContractParameter>,
		_max_items: usize,
		mapper: impl Fn(StackItem) -> U + Send,
	) -> Result<Vec<U>, ContractError> {
		let script = ScriptBuilder::build_contract_call_and_unwrap_iterator(
			&self.script_hash(),
			function,
			&params,
			_max_items as u32, // Use the max_items parameter provided to the function
			Some(CallFlags::All),
		)
		.map_err(|e| {
			ContractError::RuntimeError(format!("Failed to build iterator script: {e}"))
		})?;

		let provider = self.provider().ok_or_else(|| {
			ContractError::ProviderNotSet(
				"Provider is required for contract invocations".to_string(),
			)
		})?;

		let output = provider.invoke_script(script.to_hex_string(), vec![]).await?;

		self.throw_if_fault_state(&output)?;

		let stack_item = output
			.get_first_stack_item()
			.map_err(|e| ContractError::InvalidResponse(e.to_string()))?;

		let array = stack_item
			.as_array()
			.ok_or_else(|| ContractError::UnexpectedReturnType("Array".to_string()))?;

		let items = array.into_iter().map(mapper).collect();

		Ok(items)
	}

	fn calc_native_contract_hash(contract_name: &str) -> Result<H160, ContractError> {
		Self::calc_contract_hash(H160::zero(), 0, contract_name)
	}

	/// Calculates a native contract hash, panicking on failure.
	///
	/// This is intended for use with known-good constants like `NeoToken`, `GasToken`, etc.,
	/// where the only failure mode would be an empty name (a programming error).
	fn calc_native_contract_hash_unchecked(contract_name: &str) -> H160 {
		Self::calc_native_contract_hash(contract_name)
			.unwrap_or_else(|e| panic!("BUG: failed to compute native contract hash for '{}': {}", contract_name, e))
	}

	fn calc_contract_hash(
		sender: H160,
		nef_checksum: u32,
		contract_name: &str,
	) -> Result<H160, ContractError> {
		if contract_name.is_empty() {
			return Err(ContractError::InvalidNeoName("Contract name cannot be empty".to_string()));
		}

		let mut script = ScriptBuilder::new();
		script
			.op_code(&[OpCode::Abort])
			.push_data(sender.to_vec())
			.push_integer(BigInt::from(nef_checksum))
			.push_data(contract_name.as_bytes().to_vec());

		Ok(ScriptHash::from_script(&script.to_bytes()))
	}

	async fn get_manifest(&self) -> ContractManifest {
		let Some(provider) = self.provider() else {
			tracing::warn!("Provider not set; returning default contract manifest");
			return ContractManifest::default();
		};

		match provider.get_contract_state(self.script_hash()).await {
			Ok(state) => state.manifest,
			Err(err) => {
				tracing::warn!(error = %err, "Failed to fetch contract manifest; returning default");
				ContractManifest::default()
			},
		}
	}
}
