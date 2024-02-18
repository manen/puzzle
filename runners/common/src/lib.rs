mod wasm_trait;
pub use wasm_trait::*;

#[cfg(test)]
mod tests;

pub fn start<E: Engine>(wasm: &[u8]) -> anyhow::Result<()> {
	let engine = E::default();
	let linker = engine.linker(wasm)?;
	// TODO link libs

	let mut instance = linker.start()?;
	let _: i32 = instance.call("puzzle_main", ())?;

	Ok(())
}
