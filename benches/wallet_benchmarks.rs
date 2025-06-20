use criterion::{black_box, criterion_group, criterion_main, Criterion};
use neo3::{
	neo_protocol::{Account, AccountTrait},
	neo_wallets::{Wallet, WalletBackup, WalletTrait},
};
use tempfile::TempDir;

fn benchmark_wallet_creation(c: &mut Criterion) {
	c.bench_function("wallet_creation", |b| b.iter(|| black_box(Wallet::new())));
}

fn benchmark_account_addition(c: &mut Criterion) {
	c.bench_function("account_addition", |b| {
		b.iter(|| {
			let mut wallet = Wallet::new();
			let account = Account::create().unwrap();
			let _: () = wallet.add_account(account);
			black_box(());
		});
	});
}

fn benchmark_wallet_encryption(c: &mut Criterion) {
	let mut wallet = Wallet::new();
	for _ in 0..10 {
		let account = Account::create().unwrap();
		wallet.add_account(account);
	}
	let password = "benchmark_password_123";

	c.bench_function("wallet_encryption_10_accounts", |b| {
		b.iter(|| {
			let mut test_wallet = wallet.clone();
			test_wallet.encrypt_accounts(password);
			black_box(());
		});
	});
}

fn benchmark_wallet_backup(c: &mut Criterion) {
	let mut wallet = Wallet::new();
	for _ in 0..5 {
		let account = Account::create().unwrap();
		wallet.add_account(account);
	}
	wallet.encrypt_accounts("test_password");

	c.bench_function("wallet_backup_5_accounts", |b| {
		b.iter_batched(
			|| {
				let temp_dir = TempDir::new().unwrap();
				temp_dir.path().join("benchmark_wallet.json")
			},
			|backup_path| {
				WalletBackup::backup(&wallet, backup_path).unwrap();
				black_box(());
			},
			criterion::BatchSize::SmallInput,
		);
	});
}

fn benchmark_wallet_recovery(c: &mut Criterion) {
	// Setup: Create a wallet backup
	let mut wallet = Wallet::new();
	for _ in 0..5 {
		let account = Account::create().unwrap();
		wallet.add_account(account);
	}
	wallet.encrypt_accounts("test_password");

	let temp_dir = TempDir::new().unwrap();
	let backup_path = temp_dir.path().join("recovery_benchmark.json");
	WalletBackup::backup(&wallet, backup_path.clone()).unwrap();

	c.bench_function("wallet_recovery_5_accounts", |b| {
		b.iter(|| black_box(WalletBackup::recover(backup_path.clone()).unwrap()));
	});
}

fn benchmark_large_wallet_operations(c: &mut Criterion) {
	// Test with larger wallets
	let mut large_wallet = Wallet::new();
	for _ in 0..100 {
		let account = Account::create().unwrap();
		large_wallet.add_account(account);
	}

	c.bench_function("large_wallet_encryption_100_accounts", |b| {
		b.iter(|| {
			let mut test_wallet = large_wallet.clone();
			test_wallet.encrypt_accounts("benchmark_password");
			black_box(());
		});
	});

	large_wallet.encrypt_accounts("test_password");

	c.bench_function("large_wallet_backup_100_accounts", |b| {
		b.iter_batched(
			|| {
				let temp_dir = TempDir::new().unwrap();
				temp_dir.path().join("large_benchmark_wallet.json")
			},
			|backup_path| {
				WalletBackup::backup(&large_wallet, backup_path).unwrap();
				black_box(());
			},
			criterion::BatchSize::SmallInput,
		);
	});
}

fn benchmark_password_verification(c: &mut Criterion) {
	let mut wallet = Wallet::new();
	let account = Account::create().unwrap();
	wallet.add_account(account);
	wallet.encrypt_accounts("correct_password");

	c.bench_function("password_verification_correct", |b| {
		b.iter(|| black_box(wallet.verify_password("correct_password")));
	});

	c.bench_function("password_verification_incorrect", |b| {
		b.iter(|| black_box(wallet.verify_password("wrong_password")));
	});
}

criterion_group!(
	wallet_benches,
	benchmark_wallet_creation,
	benchmark_account_addition,
	benchmark_wallet_encryption,
	benchmark_wallet_backup,
	benchmark_wallet_recovery,
	benchmark_large_wallet_operations,
	benchmark_password_verification
);

criterion_main!(wallet_benches);
