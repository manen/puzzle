use clap::Parser;
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
	path: PathBuf,
}

fn main() {
	pretty_env_logger::init();

	let cli = Cli::parse();

	let wasm = fs::read(&cli.path).expect("failed to read file");
	wasmtime_common::start(&wasm).expect("execution failed");
}
