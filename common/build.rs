use std::fs;

fn main() -> anyhow::Result<()> {
	let hash = bash::run("git rev-parse HEAD")?;
	let msg = bash::run("git log -1 --pretty=%B")?;

	let title = "puzzle";
	let title_dbg = format!("puzzle beta ({hash})");

	let gen = format!(
		"pub const HASH: &'static str = \"{hash}\";
pub const MSG: &'static str = \"{msg}\";

#[cfg(not(debug_assertions))]
pub const TITLE: &'static str = \"{title}\";
#[cfg(debug_assertions)]
pub const TITLE: &'static str = \"{title_dbg}\";
"
	);
	fs::write("./src/gen.rs", &gen)?;

	println!("cargo:rustc-rerun-if-changed=.git/HEAD");
	Ok(())
}
