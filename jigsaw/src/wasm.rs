use crate::{Result, Runtime};
use std::{slice, str};
use wasmtime::{Caller, Linker};

pub fn jigsaw_start<S: AsMut<Runtime>>(mut caller: Caller<'_, S>) -> i32 {
	match caller.data_mut().as_mut().start() {
		Ok(_) => 0,
		Err(err) => {
			log::error!("{err}");
			-1
		}
	}
}
pub fn jigsaw_debug_text<S: AsMut<Runtime>>(
	mut caller: Caller<'_, S>,
	x: u32,
	y: u32,
	ptr_wasm: i32,
	len: u32,
) -> i32 {
	let memory = caller
		.get_export("memory")
		.expect("memory is not exported from called")
		.into_memory()
		.expect("into_memory() returned None");
	let ptr_native = unsafe { memory.data_ptr(&caller).offset(ptr_wasm as isize) };
	let msg = str::from_utf8(unsafe { slice::from_raw_parts(ptr_native, len as usize) })
		.expect("failed to convert puzzle_log message to str");

	caller.data_mut().as_mut().debug_text(x, y, msg)
}

pub trait LinkerExt {
	fn link_jigsaw(&mut self) -> Result<()>;
}
impl<S: AsMut<Runtime> + 'static> LinkerExt for Linker<S> {
	fn link_jigsaw(&mut self) -> Result<()> {
		self.func_wrap("env", "jigsaw_start", jigsaw_start)?;
		self.func_wrap("env", "jigsaw_debug_text", jigsaw_debug_text)?;

		Ok(())
	}
}
