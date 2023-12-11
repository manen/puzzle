use crate::Result;
use std::{slice, str};
use wasmtime::{Caller, Engine, Linker};

pub fn linker<T>(engine: &Engine) -> Result<Linker<T>> {
	let mut linker = Linker::new(engine);

	linker.func_wrap(
		"env",
		"puzzle_log",
		|mut caller: Caller<'_, T>, level_num: u32, ptr_wasm: i32, len: u32| {
			let memory = caller
				.get_export("memory")
				.expect("memory is not exported from called")
				.into_memory()
				.expect("into_memory() returned None");

			let level = unsafe { *(&(level_num as usize) as *const usize as *const log::Level) };

			let ptr_native = unsafe { memory.data_ptr(&caller).offset(ptr_wasm as isize) };
			let msg = str::from_utf8(unsafe { slice::from_raw_parts(ptr_native, len as usize) })
				.expect("failed to convert puzzle_log message to str");

			log::log!(level, "{}", msg);
		},
	)?;
	linker.func_wrap("env", "puzzle_log_flush", |_: Caller<'_, T>| {
		log::logger().flush();
	})?;

	Ok(linker)
}
