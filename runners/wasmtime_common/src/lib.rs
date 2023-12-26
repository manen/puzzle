use thiserror::Error;
use wasmtime::{Engine, Linker, Module, Store};

#[derive(Debug, Error)]
pub enum Error {
	#[error("wasmtime error: {0}")]
	Wasmtime(#[from] anyhow::Error),
}
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Default)]
pub struct Runtime {}

pub fn start(wasm: &[u8]) -> Result<()> {
	let engine = Engine::default();
	let module = Module::from_binary(&engine, wasm)?;

	let mut store = Store::new(&engine, Runtime::default());

	let mut linker = Linker::new(&engine);
	puzzle_log::link(&mut linker)?;

	let instance = linker.instantiate(&mut store, &module)?;

	let puzzle_main = instance.get_typed_func::<(), ()>(&mut store, "puzzle_main")?;
	puzzle_main.call(&mut store, ())?;

	let puzzle_render = instance.get_typed_func::<(), ()>(&mut store, "puzzle_render")?;

	Ok(())
}
