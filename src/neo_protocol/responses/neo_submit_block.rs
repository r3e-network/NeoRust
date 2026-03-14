use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
pub struct SubmitBlock(bool);
impl SubmitBlock {
	pub fn get_submit_block(&self) -> bool {
		self.0
	}
}
