//! Definitions of Native Contract script hashes for Neo N3.
//!
//! This module provides constants for all native contracts available on the Neo N3 blockchain.
//! These script hashes are the same across both Mainnet and Testnet.

/// Contract Management native contract script hash
pub const CONTRACT_MANAGEMENT: &str = "0xfffdc93764dbaddd97c48f252a53ea4643faa3fd";

/// Standard Library native contract script hash
pub const STD_LIB: &str = "0xacce6fd80d44e1796aa0c2c625e9e4e0ce39efc0";

/// Cryptography Library native contract script hash
pub const CRYPTO_LIB: &str = "0x726cb6e0cd8628a1350a611384688911ab75f51b";

/// Ledger native contract script hash
pub const LEDGER: &str = "0xda65b600f7124ce6c79950c1772a36403104f2be";

/// NEO Token native contract script hash
pub const NEO_TOKEN: &str = "0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5";

/// GAS Token native contract script hash
pub const GAS_TOKEN: &str = "0xd2a4cff31913016155e38e474a2c06d08be276cf";

/// Policy native contract script hash
pub const POLICY: &str = "0xcc5e4edd9f5f8dba8bb65734541df7a1c081c67b";

/// Role Management native contract script hash
pub const ROLE_MANAGEMENT: &str = "0x49cf4e5378ffcd4dec034fd98a174c5491e395e2";

/// Oracle native contract script hash
pub const ORACLE: &str = "0xfe924b7cfe89ddd271abaf7210a80a7e11178758";

/// Neo Name Service contract script hash (MainNet).
///
/// NOTE: NNS is NOT a native contract -- it is a deployed contract whose hash
/// differs between MainNet and TestNet. This constant holds the MainNet hash.
pub const NAME_SERVICE: &str = "0x7a8fcf0392cd625647907afa8e45cc66872b596b";

#[cfg(test)]
mod tests {
	use super::*;

	fn is_valid_script_hash(s: &str) -> bool {
		s.starts_with("0x") && s.len() == 42 && s[2..].chars().all(|c| c.is_ascii_hexdigit())
	}

	#[test]
	fn test_native_contract_addresses_format() {
		let hashes = [
			CONTRACT_MANAGEMENT,
			STD_LIB,
			CRYPTO_LIB,
			LEDGER,
			NEO_TOKEN,
			GAS_TOKEN,
			POLICY,
			ROLE_MANAGEMENT,
			ORACLE,
			NAME_SERVICE,
		];
		for hash in &hashes {
			assert!(is_valid_script_hash(hash), "Invalid script hash format: {}", hash);
		}
	}

	#[test]
	fn test_native_contract_known_hashes() {
		assert_eq!(NEO_TOKEN, "0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5");
		assert_eq!(GAS_TOKEN, "0xd2a4cff31913016155e38e474a2c06d08be276cf");
		assert_eq!(CONTRACT_MANAGEMENT, "0xfffdc93764dbaddd97c48f252a53ea4643faa3fd");
		assert_eq!(POLICY, "0xcc5e4edd9f5f8dba8bb65734541df7a1c081c67b");
		assert_eq!(ROLE_MANAGEMENT, "0x49cf4e5378ffcd4dec034fd98a174c5491e395e2");
		assert_eq!(ORACLE, "0xfe924b7cfe89ddd271abaf7210a80a7e11178758");
		assert_eq!(LEDGER, "0xda65b600f7124ce6c79950c1772a36403104f2be");
		assert_eq!(CRYPTO_LIB, "0x726cb6e0cd8628a1350a611384688911ab75f51b");
		assert_eq!(STD_LIB, "0xacce6fd80d44e1796aa0c2c625e9e4e0ce39efc0");
	}
}
