//! Hardfork definitions for Neo N3 blockchain.
//!
//! This module defines the various hardforks that have been implemented in the Neo N3 blockchain.
//! Each hardfork represents a set of protocol changes that are activated at a specific block height.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Represents the various hardforks implemented in the Neo N3 blockchain.
///
/// Hardforks are used to introduce new features or fix issues in the protocol.
/// Each hardfork is activated at a specific block height configured in the protocol settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Hardfork {
	/// Aspidochelone hardfork - First hardfork after Neo N3 launch
	#[serde(rename = "HF_Aspidochelone")]
	Aspidochelone = 0,

	/// Basilisk hardfork - Enhanced contract features
	#[serde(rename = "HF_Basilisk")]
	Basilisk = 1,

	/// Cockatrice hardfork - Committee change notifications, Keccak256 support
	#[serde(rename = "HF_Cockatrice")]
	Cockatrice = 2,

	/// Domovoi hardfork - Various optimizations
	#[serde(rename = "HF_Domovoi")]
	Domovoi = 3,

	/// Echidna hardfork - Notary contract, Policy updates (milliseconds per block, etc.)
	#[serde(rename = "HF_Echidna")]
	Echidna = 4,

	/// Faun hardfork - Treasury contract, Whitelist fee contracts, exec fee factor with decimals
	#[serde(rename = "HF_Faun")]
	Faun = 5,

	/// Gorgon hardfork - Upcoming protocol enhancements
	#[serde(rename = "HF_Gorgon")]
	Gorgon = 6,
}

impl Hardfork {
	/// Returns all hardforks in order.
	pub fn all() -> &'static [Hardfork] {
		&[
			Hardfork::Aspidochelone,
			Hardfork::Basilisk,
			Hardfork::Cockatrice,
			Hardfork::Domovoi,
			Hardfork::Echidna,
			Hardfork::Faun,
			Hardfork::Gorgon,
		]
	}

	/// Returns the hardfork name as used in configuration files.
	pub fn name(&self) -> &'static str {
		match self {
			Hardfork::Aspidochelone => "HF_Aspidochelone",
			Hardfork::Basilisk => "HF_Basilisk",
			Hardfork::Cockatrice => "HF_Cockatrice",
			Hardfork::Domovoi => "HF_Domovoi",
			Hardfork::Echidna => "HF_Echidna",
			Hardfork::Faun => "HF_Faun",
			Hardfork::Gorgon => "HF_Gorgon",
		}
	}

	/// Returns a short description of what this hardfork introduced.
	pub fn description(&self) -> &'static str {
		match self {
			Hardfork::Aspidochelone => "First hardfork after Neo N3 launch",
			Hardfork::Basilisk => "Enhanced contract features",
			Hardfork::Cockatrice => "Committee change notifications, Keccak256 support",
			Hardfork::Domovoi => "Various optimizations",
			Hardfork::Echidna => "Notary contract, Policy updates",
			Hardfork::Faun => "Treasury contract, Whitelist fee contracts",
			Hardfork::Gorgon => "Upcoming protocol enhancements",
		}
	}
}

impl fmt::Display for Hardfork {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.name())
	}
}

impl FromStr for Hardfork {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"hf_aspidochelone" | "aspidochelone" => Ok(Hardfork::Aspidochelone),
			"hf_basilisk" | "basilisk" => Ok(Hardfork::Basilisk),
			"hf_cockatrice" | "cockatrice" => Ok(Hardfork::Cockatrice),
			"hf_domovoi" | "domovoi" => Ok(Hardfork::Domovoi),
			"hf_echidna" | "echidna" => Ok(Hardfork::Echidna),
			"hf_faun" | "faun" => Ok(Hardfork::Faun),
			"hf_gorgon" | "gorgon" => Ok(Hardfork::Gorgon),
			_ => Err(format!("Unknown hardfork: {}", s)),
		}
	}
}

impl TryFrom<u8> for Hardfork {
	type Error = String;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Hardfork::Aspidochelone),
			1 => Ok(Hardfork::Basilisk),
			2 => Ok(Hardfork::Cockatrice),
			3 => Ok(Hardfork::Domovoi),
			4 => Ok(Hardfork::Echidna),
			5 => Ok(Hardfork::Faun),
			6 => Ok(Hardfork::Gorgon),
			_ => Err(format!("Invalid hardfork value: {}", value)),
		}
	}
}

impl From<Hardfork> for u8 {
	fn from(hardfork: Hardfork) -> Self {
		hardfork as u8
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_hardfork_ordering() {
		assert!(Hardfork::Aspidochelone < Hardfork::Basilisk);
		assert!(Hardfork::Basilisk < Hardfork::Cockatrice);
		assert!(Hardfork::Cockatrice < Hardfork::Domovoi);
		assert!(Hardfork::Domovoi < Hardfork::Echidna);
		assert!(Hardfork::Echidna < Hardfork::Faun);
		assert!(Hardfork::Faun < Hardfork::Gorgon);
	}

	#[test]
	fn test_hardfork_from_str() {
		assert_eq!(Hardfork::from_str("HF_Aspidochelone").unwrap(), Hardfork::Aspidochelone);
		assert_eq!(Hardfork::from_str("aspidochelone").unwrap(), Hardfork::Aspidochelone);
		assert_eq!(Hardfork::from_str("HF_Faun").unwrap(), Hardfork::Faun);
	}

	#[test]
	fn test_hardfork_display() {
		assert_eq!(Hardfork::Aspidochelone.to_string(), "HF_Aspidochelone");
		assert_eq!(Hardfork::Faun.to_string(), "HF_Faun");
	}

	#[test]
	fn test_hardfork_try_from_u8() {
		assert_eq!(Hardfork::try_from(0).unwrap(), Hardfork::Aspidochelone);
		assert_eq!(Hardfork::try_from(5).unwrap(), Hardfork::Faun);
		assert!(Hardfork::try_from(255).is_err());
	}

	#[test]
	fn test_hardfork_all() {
		let all = Hardfork::all();
		assert_eq!(all.len(), 7);
		assert_eq!(all[0], Hardfork::Aspidochelone);
		assert_eq!(all[6], Hardfork::Gorgon);
	}
}
