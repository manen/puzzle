use std::{env, fs};

fn main() -> anyhow::Result<()> {
	let out_dir = env::var("OUT_DIR")?;
	let gen_path = format!("{out_dir}/gen.rs");

	let hash = bash::run("git rev-parse HEAD")?;
	let msg = bash::run("git log -1 --pretty=%B")?;

	let title = format!("puzzle beta ({hash})");

	let gen = format!(
		"pub const HASH: &'static str = \"{hash}\";
pub const MSG: &'static str = \"{msg}\";

pub const TITLE: &'static str = \"{title}\";
"
	);
	fs::write(&gen_path, &gen)?;

	let pwd = bash::run("run pwd")?;
	println!("cargo:rerun-if-changed=build.rs");
	println!("cargo:rerun-if-changed={pwd}/Cargo.lock");
	Ok(())
}
