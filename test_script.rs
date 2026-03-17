use std::process::Command;
use std::time::Instant;

fn main() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run").arg("-p").arg("neo-cli").arg("--").arg("network").arg("add").arg("--url").arg("http://seed1.ngd.network:10332").arg("--name").arg("test-node");
    
    let start = Instant::now();
    let _output = cmd.output().unwrap();
    println!("Took {:?}", start.elapsed());
}
