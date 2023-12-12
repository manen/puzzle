use clap::Parser;
use std::{
	env, fs, os,
	path::{Path, PathBuf},
	process::{self, Command},
};

#[derive(Parser, Debug)]
#[command(name = "Puzzle")]
#[command(author = "manen")]
enum Cli {
	Run {
		/// wasm binary path
		path: PathBuf,
	},
	/// compile and reload
	CargoWatch {
		/// application working directory path
		#[arg(short = 'C', long)]
		build_dir: Option<PathBuf>,
		/// rust target directory
		#[arg(short, long)]
		target_dir: Option<PathBuf>,
	},
}

fn main() {
	pretty_env_logger::init();

	let cli = Cli::parse();
	match cli {
		Cli::Run { path } => run(&path),
		Cli::CargoWatch {
			build_dir,
			target_dir,
		} => {
			let build_dir = build_dir.unwrap_or_else(|| env::current_dir().unwrap());
			let target_dir = target_dir.unwrap_or_else(|| build_dir.join("target"));
			let out_path = format!(
				"{}/wasm32-unknown-unknown/debug/{}.wasm",
				target_dir.display(),
				build_dir.file_name().unwrap().to_string_lossy()
			);

			let cargo_path = pathsearch::find_executable_in_path("cargo").unwrap();

			let manifest_path = build_dir.join("Cargo.toml");
			// Command::new(cargo_path)
			// 	.args([
			// 		"build",
			// 		"--manifest_path",
			// 		&manifest_path,
			// 		"--target=wasm32-unknown-unknown",
			// 	])
			// 	.spawn()
			// 	.unwrap()
			// 	.wait()
			// 	.unwrap();

			let arg0 = env::args().nth(0).unwrap();

			let manifest_path_d = manifest_path.display();
			let cmd = format!(
				"cargo build --manifest-path {manifest_path_d} --target=wasm32-unknown-unknown && {arg0} run {out_path}"
			);

			// TODO
			//
			// this whole cargo watch thing will be rewritten if the api develops more and applications can
			// be killed safely

			let exit_code = Command::new(cargo_path)
				.args(["watch", "-s", &cmd])
				.spawn()
				.unwrap()
				.wait()
				.unwrap()
				.code()
				.unwrap();
			process::exit(exit_code);
		}
	};
}

fn run(path: &Path) {
	let wasm = fs::read(path).expect("failed to read file");
	wasmtime_common::start(&wasm).expect("execution failed");
}
