use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use neo3::neo_builder::{GasEstimator, ScriptBuilder};
use neo3::neo_types::OpCode;
use num_bigint::BigInt;

fn benchmark_script_building(c: &mut Criterion) {
	let mut group = c.benchmark_group("script_building");

	// Benchmark simple script building
	group.bench_function("simple_script", |b| {
		b.iter(|| {
			ScriptBuilder::new()
				.push_integer(std::hint::black_box(BigInt::from(42)))
				.push_integer(std::hint::black_box(BigInt::from(13)))
				.op_code(&[OpCode::Add])
				.to_bytes()
		})
	});

	// Benchmark complex script building
	group.bench_function("complex_script", |b| {
		b.iter(|| {
			let mut builder = ScriptBuilder::new();
			for i in 0..100 {
				builder.push_integer(std::hint::black_box(BigInt::from(i)));
			}
			builder.push_integer(std::hint::black_box(BigInt::from(100)));
			builder.pack().to_bytes()
		})
	});

	// Benchmark script with strings
	group.bench_function("string_script", |b| {
		let test_string = "Hello, Neo Blockchain!";
		let test_string_bytes = test_string.as_bytes().to_vec();
		let world_bytes = "World".as_bytes().to_vec();
		b.iter(|| {
			ScriptBuilder::new()
				.push_data(std::hint::black_box(test_string_bytes.clone()))
				.push_data(std::hint::black_box(world_bytes.clone()))
				.op_code(&[OpCode::Cat])
				.to_bytes()
		})
	});

	group.finish();
}

fn benchmark_gas_calculations(c: &mut Criterion) {
	let mut group = c.benchmark_group("gas_calculations");

	// Benchmark accuracy calculation
	group.bench_function("accuracy_calculation", |b| {
		b.iter(|| {
			GasEstimator::calculate_estimation_accuracy(
				std::hint::black_box(1100),
				std::hint::black_box(1000),
			)
		})
	});

	// Benchmark with different gas values
	for gas_value in [100, 1_000, 10_000, 100_000, 1_000_000].iter() {
		group.bench_with_input(
			BenchmarkId::new("calculate_margin", gas_value),
			gas_value,
			|b, &gas| {
				b.iter(|| {
					let base = std::hint::black_box(gas);
					let margin_percent = std::hint::black_box(15);
					let margin = (base as f64 * (margin_percent as f64 / 100.0)) as i64;
					base + margin
				})
			},
		);
	}

	group.finish();
}

fn benchmark_script_sizes(c: &mut Criterion) {
	let mut group = c.benchmark_group("script_sizes");

	// Benchmark different script sizes
	for size in [10, 50, 100, 500, 1000].iter() {
		group.bench_with_input(BenchmarkId::new("build_script_size", size), size, |b, &size| {
			b.iter(|| {
				let mut builder = ScriptBuilder::new();
				for i in 0..size {
					builder.push_integer(std::hint::black_box(BigInt::from(i)));
				}
				builder.to_bytes()
			})
		});
	}

	group.finish();
}

fn benchmark_opcode_emission(c: &mut Criterion) {
	let mut group = c.benchmark_group("opcode_emission");

	// Benchmark single opcode emission
	group.bench_function("single_opcode", |b| {
		b.iter(|| {
			let opcode = std::hint::black_box(OpCode::Nop);
			ScriptBuilder::new().op_code(&[opcode]).to_bytes()
		})
	});

	// Benchmark multiple opcode emission
	group.bench_function("multiple_opcodes", |b| {
		b.iter(|| {
			ScriptBuilder::new()
				.op_code(&[
					std::hint::black_box(OpCode::Push1),
					std::hint::black_box(OpCode::Push2),
					std::hint::black_box(OpCode::Add),
					std::hint::black_box(OpCode::Push3),
					std::hint::black_box(OpCode::Mul),
				])
				.to_bytes()
		})
	});

	// Benchmark opcode with parameters
	group.bench_function("opcode_with_params", |b| {
		let syscall_arg = vec![0u8; 4];
		b.iter(|| {
			ScriptBuilder::new()
				.op_code_with_arg(OpCode::Syscall, std::hint::black_box(syscall_arg.clone()))
				.to_bytes()
		})
	});

	group.finish();
}

criterion_group!(
	benches,
	benchmark_script_building,
	benchmark_gas_calculations,
	benchmark_script_sizes,
	benchmark_opcode_emission
);
criterion_main!(benches);
