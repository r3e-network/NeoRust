#[cfg(test)]
mod proptest_tests {
    use proptest::prelude::*;
    use crate::neo_crypto::{
        base58check_encode, base58check_decode,
        KeyPair,
        hash::HashableForVec,
    };

    // Property: base58check encoding and decoding should be reversible
    proptest! {
        #[test]
        fn prop_base58check_roundtrip(bytes in prop::collection::vec(any::<u8>(), 0..1000)) {
            let encoded = base58check_encode(&bytes);
            let decoded = base58check_decode(&encoded);
            prop_assert_eq!(decoded, Some(bytes));
        }
    }

    // Property: hash functions should produce consistent output
    proptest! {
        #[test]
        fn prop_sha256_consistent(input in prop::collection::vec(any::<u8>(), 0..1000)) {
            let hash1 = input.hash256();
            let hash2 = input.hash256();
            prop_assert_eq!(hash1, hash2);
        }

        #[test]
        fn prop_sha256_output_size(input in prop::collection::vec(any::<u8>(), 0..1000)) {
            let hash = input.hash256();
            prop_assert_eq!(hash.len(), 32);
        }

        #[test]
        fn prop_ripemd160_output_size(input in prop::collection::vec(any::<u8>(), 0..1000)) {
            let hash = input.ripemd160();
            prop_assert_eq!(hash.len(), 20);
        }

        #[test]
        fn prop_hash160_output_size(input in prop::collection::vec(any::<u8>(), 0..1000)) {
            let hash = input.sha256_ripemd160();
            prop_assert_eq!(hash.len(), 20);
        }

        #[test]
        fn prop_double_sha256_output_size(input in prop::collection::vec(any::<u8>(), 0..1000)) {
            let hash = input.double_sha256();
            prop_assert_eq!(hash.len(), 32);
        }
    }

    // Property: different inputs should produce different hashes (collision resistance)
    proptest! {
        #[test]
        fn prop_sha256_collision_resistance(
            input1 in prop::collection::vec(any::<u8>(), 1..100),
            input2 in prop::collection::vec(any::<u8>(), 1..100)
        ) {
            prop_assume!(input1 != input2);
            let hash1 = input1.hash256();
            let hash2 = input2.hash256();
            prop_assert_ne!(hash1, hash2);
        }
    }

    // Property: KeyPair generation should be deterministic from private key
    proptest! {
        #[test]
        fn prop_keypair_deterministic(seed in prop::array::uniform32(any::<u8>())) {
			// Skip seeds that are invalid private keys (0 or >= curve order)
			if let Ok(keypair1) = KeyPair::from_private_key(&seed) {
				let keypair2 = KeyPair::from_private_key(&seed).unwrap();
				prop_assert_eq!(keypair1.private_key_bytes().unwrap(), keypair2.private_key_bytes().unwrap());
				prop_assert_eq!(keypair1.public_key(), keypair2.public_key());
			}
		}
    }

    // Property: signing and verification should work correctly
    proptest! {
        #[test]
        fn prop_sign_verify_roundtrip(
            seed in prop::array::uniform32(any::<u8>()),
            message in prop::collection::vec(any::<u8>(), 1..1000)
        ) {
            if let Ok(keypair) = KeyPair::from_private_key(&seed) {
                let signature = keypair.sign(&message).unwrap();
                let is_valid = keypair.verify(&message, &signature).unwrap();
                prop_assert!(is_valid);
            }
        }

        #[test]
        fn prop_sign_verify_wrong_message_fails(
            seed in prop::array::uniform32(any::<u8>()),
            message1 in prop::collection::vec(any::<u8>(), 1..100),
            message2 in prop::collection::vec(any::<u8>(), 1..100)
        ) {
            prop_assume!(message1 != message2);
            if let Ok(keypair) = KeyPair::from_private_key(&seed) {
                let signature = keypair.sign(&message1).unwrap();
                let is_valid = keypair.verify(&message2, &signature).unwrap_or(false);
                prop_assert!(!is_valid);
            }
        }
    }
}
