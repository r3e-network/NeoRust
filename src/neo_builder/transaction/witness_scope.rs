use crate::builder::BuilderError;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(
	Display, EnumString, TryFromPrimitive, Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum WitnessScope {
	#[strum(serialize = "None")]
	None = 0x00,
	#[strum(serialize = "CalledByEntry")]
	CalledByEntry = 0x01,
	#[strum(serialize = "CustomContracts")]
	CustomContracts = 0x10,
	#[strum(serialize = "CustomGroups")]
	CustomGroups = 0x20,
	#[strum(serialize = "WitnessRules")]
	WitnessRules = 0x40,
	#[strum(serialize = "Global")]
	Global = 0x80,
}

impl WitnessScope {
	pub fn byte_repr(&self) -> u8 {
		match self {
			WitnessScope::None => 0x00,
			WitnessScope::CalledByEntry => 0x01,
			WitnessScope::CustomContracts => 0x10,
			WitnessScope::CustomGroups => 0x20,
			WitnessScope::WitnessRules => 0x40,
			WitnessScope::Global => 0x80,
		}
	}

	/// Validates that scope combinations are legal per Neo N3 protocol.
	/// Global and None scopes must not be combined with any other scope.
	pub fn validate(scopes: &[WitnessScope]) -> Result<(), BuilderError> {
		if scopes.len() > 1 {
			if scopes.contains(&WitnessScope::Global) {
				return Err(BuilderError::SignerConfiguration(
					"Global scope cannot be combined with other scopes".to_string(),
				));
			}
			if scopes.contains(&WitnessScope::None) {
				return Err(BuilderError::SignerConfiguration(
					"None scope cannot be combined with other scopes".to_string(),
				));
			}
		}
		Ok(())
	}

	pub fn combine(scopes: &[Self]) -> u8 {
		let mut flags = 0;
		for scope in scopes {
			flags |= scope.byte_repr();
		}
		flags
	}

	// Split bit flags
	pub fn split(flags: u8) -> Vec<Self> {
		let mut scopes = Vec::new();

		if flags == 0x00 {
			scopes.push(Self::None);
			return scopes;
		}
		if flags & Self::CalledByEntry.byte_repr() != 0 {
			scopes.push(Self::CalledByEntry);
		}
		if flags & Self::CustomContracts.byte_repr() != 0 {
			scopes.push(Self::CustomContracts);
		}
		if flags & Self::CustomGroups.byte_repr() != 0 {
			scopes.push(Self::CustomGroups);
		}
		if flags & Self::WitnessRules.byte_repr() != 0 {
			scopes.push(Self::WitnessRules);
		}
		if flags & Self::Global.byte_repr() != 0 {
			scopes.push(Self::Global);
		}

		scopes
	}
}
