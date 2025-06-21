use ethers::{prelude::Abigen, solc::Solc};
use eyre::Result;

fn main() -> Result<()> {
	// Use args_os for more secure argument handling
	let args: Vec<String> = std::env::args_os()
		.skip(1) // skip program name
		.map(|arg| arg.to_string_lossy().to_string())
		.collect();

	let contract_name = args.get(0).cloned().unwrap_or_else(|| "SimpleStorage".to_owned());
	let contract: String = args
		.get(1)
		.cloned()
		.unwrap_or_else(|| "examples/contracts/examples/contracts/contract.sol".to_owned());

	println!("Generating bindings for {contract}\n");

	// compile it
	let abi = if contract.ends_with(".sol") {
		let contracts = Solc::default().compile_source(&contract)?;
		let abi = contracts.get(&contract, &contract_name).unwrap().abi.unwrap();
		serde_json::to_string(abi).unwrap()
	} else {
		contract
	};

	let bindings = Abigen::new(&contract_name, abi)?.generate()?;

	// print to stdout if no output arg is given
	if let Some(output_path) = args.get(2) {
		bindings.write_to_file(output_path)?;
	} else {
		bindings.write(&mut std::io::stdout())?;
	}

	Ok(())
}
