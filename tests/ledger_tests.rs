#[cfg(feature = "ledger")]
mod tests {
	use neo3::neo_wallets::HDPath;

	#[test]
	fn test_hdpath_to_vec() {
		// Test LedgerLive path
		let path = HDPath::LedgerLive(0);
		assert_eq!(path.to_vec(), vec![44 + 0x8000_0000, 888 + 0x8000_0000, 0x8000_0000, 0, 0]);

		// Test Legacy path
		let path = HDPath::Legacy(1);
		assert_eq!(path.to_vec(), vec![44 + 0x8000_0000, 888 + 0x8000_0000, 0x8000_0000, 1]);

		// Test Custom path
		let path = HDPath::Custom(vec![1, 2, 3, 4, 5]);
		assert_eq!(path.to_vec(), vec![1, 2, 3, 4, 5]);
	}
}
