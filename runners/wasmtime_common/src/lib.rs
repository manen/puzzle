mod stdlib;

use thiserror::Error;
use wasmtime::{Engine, Module, Store};

#[derive(Debug, Error)]
pub enum Error {
	#[error("wasmtime error: {0}")]
	Wasmtime(#[from] anyhow::Error),
	#[error("jigsaw error: {0}")]
	Jigsaw(#[from] jigsaw::Error),
}
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Default)]
pub struct Runtime {
	jigsaw: jigsaw::Runtime,
}
impl AsMut<jigsaw::Runtime> for Runtime {
	fn as_mut(&mut self) -> &mut jigsaw::Runtime {
		&mut self.jigsaw
	}
}

pub fn start(wasm: &[u8]) -> Result<()> {
	let engine = Engine::default();
	let module = Module::from_binary(&engine, wasm)?;

	let mut store = Store::new(&engine, Runtime::default());

	let linker = stdlib::linker(&engine)?;
	let instance = linker.instantiate(&mut store, &module)?;

	let puzzle_main = instance.get_typed_func::<(), ()>(&mut store, "puzzle_main")?;
	puzzle_main.call(&mut store, ())?;

	let puzzle_render = instance.get_typed_func::<(), ()>(&mut store, "puzzle_render")?;

	match &store.data().jigsaw {
		jigsaw::Runtime::Uninit => Ok(()),
		jigsaw::Runtime::Init(_) => {
			{
				while !store.data_mut().jigsaw.runtime()?.should_close() {
					store.data_mut().jigsaw.runtime()?.frame();
					puzzle_render.call(&mut store, ())?;
					store.data_mut().jigsaw.runtime()?.frame_post();
				}
			}

			store.data_mut().jigsaw.uninit()?;
			Ok(())
		}
	}
}
