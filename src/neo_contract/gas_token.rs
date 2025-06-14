use async_trait::async_trait;
use primitive_types::H160;
use serde::{Deserialize, Serialize};

use crate::{
	neo_clients::{JsonRpcProvider, RpcClient},
	neo_contract::{
		traits::{FungibleTokenTrait, SmartContractTrait, TokenTrait},
		ContractError,
	},
	neo_types::{
		serde_with_utils::{deserialize_script_hash, serialize_script_hash},
		NNSName, ScriptHash,
	},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasToken<'a, P: JsonRpcProvider> {
	#[serde(deserialize_with = "deserialize_script_hash")]
	#[serde(serialize_with = "serialize_script_hash")]
	script_hash: ScriptHash,
	#[serde(skip_serializing_if = "Option::is_none")]
	total_supply: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	decimals: Option<u8>,
	#[serde(skip_serializing_if = "Option::is_none")]
	symbol: Option<String>,
	#[serde(skip)]
	provider: Option<&'a RpcClient<P>>,
}

impl<'a, P: JsonRpcProvider> GasToken<'a, P> {
	pub const NAME: &'static str = "GasToken";
	pub const DECIMALS: u8 = 8;
	pub const SYMBOL: &'static str = "GAS";

	pub fn new(provider: Option<&'a RpcClient<P>>) -> Self {
		Self {
			script_hash: Self::calc_native_contract_hash(Self::NAME).unwrap(),
			total_supply: None,
			decimals: Some(Self::DECIMALS),
			symbol: Some(Self::SYMBOL.to_string()),
			provider,
		}
	}
}

#[async_trait]
impl<'a, P: JsonRpcProvider> TokenTrait<'a, P> for GasToken<'a, P> {
	fn total_supply(&self) -> Option<u64> {
		self.total_supply
	}

	fn set_total_supply(&mut self, total_supply: u64) {
		self.total_supply = Option::from(total_supply);
	}

	fn decimals(&self) -> Option<u8> {
		self.decimals
	}

	fn set_decimals(&mut self, decimals: u8) {
		self.decimals = Option::from(decimals);
	}

	fn symbol(&self) -> Option<String> {
		self.symbol.clone()
	}

	fn set_symbol(&mut self, symbol: String) {
		self.symbol = Option::from(symbol);
	}

	async fn resolve_nns_text_record(&self, _name: &NNSName) -> Result<H160, ContractError> {
		Err(ContractError::UnsupportedOperation(
			"GAS token does not support NNS text record resolution".to_string(),
		))
	}
}

#[async_trait]
impl<'a, P: JsonRpcProvider> SmartContractTrait<'a> for GasToken<'a, P> {
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

#[async_trait]
impl<'a, P: JsonRpcProvider> FungibleTokenTrait<'a, P> for GasToken<'a, P> {}
