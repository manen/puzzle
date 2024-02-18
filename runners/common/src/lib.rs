use wasm_trait::*;

pub fn start<E: Engine>(wasm: &[u8]) -> anyhow::Result<()> {
	let engine = E::default();
	let linker = engine.linker(wasm)?;
	// TODO link libs

	let mut instance = linker.start()?;
	let _: i32 = instance.call("puzzle_main", ())?;

	Ok(())
}
