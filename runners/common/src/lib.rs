mod stdlib;

use thiserror::Error;
use wasmtime::{Engine, Module, Store};

#[derive(Debug, Error)]
pub enum Error {
	#[error("wasmtime error: {0}")]
	Wasmtime(#[from] anyhow::Error),
}
pub type Result<T> = std::result::Result<T, Error>;

pub fn start(wasm: &[u8]) -> Result<()> {
	let engine = Engine::default();
	let module = Module::from_binary(&engine, wasm)?;

	let mut store = Store::new(&engine, ());

	let linker = stdlib::linker(&engine)?;
	let instance = linker.instantiate(&mut store, &module)?;

	let run = instance.get_typed_func::<(), ()>(&mut store, "puzzle_main")?;
	run.call(&mut store, ())?;

	Ok(())
}
