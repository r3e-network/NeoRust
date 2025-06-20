use criterion::{black_box, criterion_group, criterion_main, Criterion};
use neo3::{
	neo_crypto::{HashableForVec, KeyPair},
	neo_protocol::{Account, AccountTrait},
};

fn benchmark_key_generation(c: &mut Criterion) {
	c.bench_function("key_pair_generation", |b| {
		b.iter(|| black_box(KeyPair::new_random()))
	});
}

fn benchmark_account_creation(c: &mut Criterion) {
	c.bench_function("account_creation", |b| b.iter(|| black_box(Account::create().unwrap())));
}

fn benchmark_signature_creation(c: &mut Criterion) {
	let key_pair = KeyPair::new_random();
	let message = b"Hello Neo N3 Blockchain! This is a benchmark message.";
	let message_hash = message.hash256();

	c.bench_function("signature_creation", |b| {
		b.iter(|| black_box(key_pair.private_key.sign_prehash(&message_hash).unwrap()))
	});
}

fn benchmark_signature_verification(c: &mut Criterion) {
	let key_pair = KeyPair::new_random();
	let message = b"Hello Neo N3 Blockchain! This is a benchmark message.";
	let message_hash = message.hash256();
	let signature = key_pair.private_key.sign_prehash(&message_hash).unwrap();

	c.bench_function("signature_verification", |b| {
		b.iter(|| black_box(key_pair.public_key.verify(&message_hash, &signature).is_ok()))
	});
}

fn benchmark_hash_operations(c: &mut Criterion) {
	let data = vec![0u8; 1024]; // 1KB of data

	c.bench_function("hash256_1kb", |b| b.iter(|| black_box(data.hash256())));

	let large_data = vec![0u8; 1024 * 1024]; // 1MB of data
	c.bench_function("hash256_1mb", |b| b.iter(|| black_box(large_data.hash256())));
}

fn benchmark_address_generation(c: &mut Criterion) {
	let key_pair = KeyPair::new_random();

	c.bench_function("address_from_public_key", |b| {
		b.iter(|| {
			let account = Account::from_key_pair(key_pair.clone(), None, None).unwrap();
			black_box(account.get_address())
		})
	});
}

fn benchmark_wif_operations(c: &mut Criterion) {
	let key_pair = KeyPair::new_random();
	let wif = key_pair.export_as_wif();

	c.bench_function("wif_export", |b| b.iter(|| black_box(key_pair.export_as_wif())));

	c.bench_function("wif_import", |b| b.iter(|| black_box(Account::from_wif(&wif).unwrap())));
}

criterion_group!(
	crypto_benches,
	benchmark_key_generation,
	benchmark_account_creation,
	benchmark_signature_creation,
	benchmark_signature_verification,
	benchmark_hash_operations,
	benchmark_address_generation,
	benchmark_wif_operations
);

criterion_main!(crypto_benches);
