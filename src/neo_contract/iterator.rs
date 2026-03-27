use std::{fmt, sync::Arc};

use crate::neo_types::StackItem;
use crate::{
	neo_clients::{APITrait, JsonRpcProvider, RpcClient},
	neo_contract::ContractError,
};

pub struct NeoIterator<'a, T, P: JsonRpcProvider> {
	session_id: String,
	iterator_id: String,
	mapper: Arc<dyn Fn(StackItem) -> Result<T, ContractError> + Send + Sync>,
	provider: Option<&'a RpcClient<P>>,
}

impl<'a, T, P: JsonRpcProvider> fmt::Debug for NeoIterator<'a, T, P> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("NeoIterator")
			.field("session_id", &self.session_id)
			.field("iterator_id", &self.iterator_id)
			// For the mapper, you can decide what to print. Here, we just print a static string.
			.field("mapper", &"<function>")
			.finish()
	}
}

impl<'a, T, P: JsonRpcProvider> NeoIterator<'a, T, P> {
	pub fn new(
		session_id: String,
		iterator_id: String,
		mapper: Arc<dyn Fn(StackItem) -> Result<T, ContractError> + Send + Sync>,
		provider: Option<&'a RpcClient<P>>,
	) -> Self {
		Self { session_id, iterator_id, mapper, provider }
	}

	pub async fn traverse(&self, count: i32) -> Result<Vec<T>, ContractError> {
		if count < 0 {
			return Err(ContractError::InvalidArgError(
				"Iterator traversal count must be non-negative".to_string(),
			));
		}

		let provider = self.provider.ok_or_else(|| {
			ContractError::ProviderNotSet("Provider is required for iterator traversal".to_string())
		})?;

		let result = provider
			.traverse_iterator(self.session_id.clone(), self.iterator_id.clone(), count as u32)
			.await?;
		let mapped = result
			.iter()
			.cloned()
			.map(|item| (self.mapper)(item))
			.collect::<Result<Vec<_>, _>>()?;
		Ok(mapped)
	}

	pub async fn terminate_session(&self) -> Result<(), ContractError> {
		let provider = self.provider.ok_or_else(|| {
			ContractError::ProviderNotSet(
				"Provider is required for iterator session termination".to_string(),
			)
		})?;
		let _ = provider.terminate_session(&self.session_id).await?;
		Ok(())
	}
}
