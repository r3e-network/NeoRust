use async_trait::async_trait;
use primitive_types::H160;
use serde::{Deserialize, Serialize};

use crate::{
	builder::TransactionBuilder,
	neo_clients::{APITrait, JsonRpcProvider, RpcClient},
	neo_contract::{ContractError, SmartContractTrait},
	ContractIdentifiers,
};
use neo3::prelude::*;

/// A struct representing contract management functionalities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractManagement<'a, P: JsonRpcProvider> {
	#[serde(deserialize_with = "deserialize_script_hash")]
	#[serde(serialize_with = "serialize_script_hash")]
	script_hash: ScriptHash,
	#[serde(skip)]
	provider: Option<&'a RpcClient<P>>,
}

impl<'a, P: JsonRpcProvider + 'static> ContractManagement<'a, P> {
	pub fn new(script_hash: H160, provider: Option<&'a RpcClient<P>>) -> Self {
		Self { script_hash, provider }
	}

	pub async fn get_minimum_deployment_fee(&self) -> Result<u64, ContractError> {
		let output = self.call_invoke_function("getMinimumDeploymentFee", vec![], vec![]).await?;
		self.throw_if_fault_state(&output)?;

		let item = output
			.get_first_stack_item()
			.map_err(|e| ContractError::InvalidResponse(e.to_string()))?;
		let value = item
			.as_int()
			.ok_or_else(|| ContractError::UnexpectedReturnType("Int".to_string()))?;

		u64::try_from(value).map_err(|_| {
			ContractError::InvalidResponse("Minimum deployment fee cannot be negative".to_string())
		})
	}

	pub async fn set_minimum_deployment_fee(&self, fee: u64) -> Result<u64, ContractError> {
		let output = self
			.call_invoke_function("setMinimumDeploymentFee", vec![fee.into()], vec![])
			.await?;
		self.throw_if_fault_state(&output)?;

		let item = output
			.get_first_stack_item()
			.map_err(|e| ContractError::InvalidResponse(e.to_string()))?;
		let value = item
			.as_int()
			.ok_or_else(|| ContractError::UnexpectedReturnType("Int".to_string()))?;

		u64::try_from(value).map_err(|_| {
			ContractError::InvalidResponse("Minimum deployment fee cannot be negative".to_string())
		})
	}

	pub async fn get_contract(&self, hash: H160) -> Result<ContractState, ContractError> {
		let provider = self.provider.ok_or_else(|| {
			ContractError::ProviderNotSet("Provider is required for ContractManagement".to_string())
		})?;
		Ok(provider.get_contract_state(hash).await?)
	}

	pub async fn get_contract_by_id(&self, id: u32) -> Result<ContractState, ContractError> {
		let hash = self.get_contract_hash_by_id(id).await?;
		self.get_contract(hash).await
	}

	pub async fn get_contract_hash_by_id(&self, id: u32) -> Result<ScriptHash, ContractError> {
		let result = self.call_invoke_function("getContractById", vec![id.into()], vec![]).await?;
		self.throw_if_fault_state(&result)?;

		let item = result
			.get_first_stack_item()
			.map_err(|e| ContractError::InvalidResponse(e.to_string()))?;

		let bytes = item
			.as_bytes()
			.ok_or_else(|| ContractError::UnexpectedReturnType("ByteString".to_string()))?;
		if bytes.len() != 20 {
			return Err(ContractError::InvalidScriptHash(format!(
				"Expected 20 bytes for ScriptHash, got {}",
				bytes.len()
			)));
		}
		Ok(ScriptHash::from_slice(&bytes))
	}

	pub async fn get_contract_hashes(&self) -> Result<ContractIdentifiers, ContractError> {
		let result = self.call_invoke_function("getContractHashes", vec![], vec![]).await?;
		self.throw_if_fault_state(&result)?;

		ContractIdentifiers::from_invocation_result(result)
			.map_err(|e| ContractError::InvalidResponse(e.to_string()))
	}

	pub async fn has_method(
		&self,
		hash: H160,
		method: &str,
		params: usize,
	) -> Result<bool, ContractError> {
		let result = self
			.call_invoke_function(
				"hasMethod",
				vec![hash.into(), method.into(), params.into()],
				vec![],
			)
			.await?;
		self.throw_if_fault_state(&result)?;

		let item = result
			.get_first_stack_item()
			.map_err(|e| ContractError::InvalidResponse(e.to_string()))?;
		item.as_bool()
			.ok_or_else(|| ContractError::UnexpectedReturnType("Bool".to_string()))
	}

	pub async fn deploy(
		&self,
		nef: &NefFile,
		manifest: &[u8],
		data: Option<ContractParameter>,
	) -> Result<TransactionBuilder<'_, P>, ContractError> {
		let params = vec![nef.into(), manifest.into(), data.unwrap_or_else(ContractParameter::any)];
		self.invoke_function("deploy", params).await
	}
}

#[async_trait]
impl<'a, P: JsonRpcProvider> SmartContractTrait<'a> for ContractManagement<'a, P> {
	type P = P;

	fn script_hash(&self) -> H160 {
		self.script_hash
	}

	fn set_script_hash(&mut self, script_hash: H160) {
		self.script_hash = script_hash;
	}

	fn provider(&self) -> Option<&RpcClient<P>> {
		self.provider
	}
}
