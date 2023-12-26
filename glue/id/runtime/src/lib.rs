pub use id_common::*;
use std::{slice, str};
use thiserror::Error;
use wasmtime::Caller;

#[derive(Error, Debug)]
pub enum RtError {
	#[error("{0}")]
	Common(#[from] Error),
	#[error("wasmtime error: {0}")]
	Wasmtime(#[from] wasmtime::Error),
	#[error("memory isn't exported from wasmtime instance")]
	NoMemory,
	#[error("memory.into_memory() failed")]
	IntoMemory,
	#[error("{0}")]
	Utf8Error(#[from] str::Utf8Error),
}
pub type Result<T> = std::result::Result<T, RtError>;

pub fn ensure_app_api_version<T>(
	store: &mut wasmtime::Store<T>,
	instance: &wasmtime::Instance,
) -> Result<()> {
	let runner = puzzle_common::API_VERSION;
	let app = app(store, instance)?.api_version;
	Ok(api_check(runner, app)?)
}

pub fn app<T>(mut store: &mut wasmtime::Store<T>, instance: &wasmtime::Instance) -> Result<App> {
	let name_len: u32 = instance
		.get_typed_func(&mut store, "puzzle_id_name_len")?
		.call(&mut store, ())?;
	let name_ptr: u32 = instance
		.get_typed_func(&mut store, "puzzle_id_name")?
		.call(&mut store, ())?;
	let name = instance
		.get_export(&mut store, "memory")
		.ok_or(RtError::NoMemory)?
		.into_memory()
		.ok_or(RtError::IntoMemory)?
		.data_ptr(&mut store);
	let name = unsafe { name.offset(name_ptr as isize) };
	let name = unsafe { slice::from_raw_parts(name, name_len as usize) };
	let name = str::from_utf8(name)?;

	let version_major: u32 = instance
		.get_typed_func(&mut store, "puzzle_id_version_major")?
		.call(&mut store, ())?;
	let version_minor: u32 = instance
		.get_typed_func(&mut store, "puzzle_id_version_minor")?
		.call(&mut store, ())?;
	let version_patch: u32 = instance
		.get_typed_func(&mut store, "puzzle_id_version_patch")?
		.call(&mut store, ())?;
	let version = (version_major, version_minor, version_patch);

	let api_version_major: u32 = instance
		.get_typed_func(&mut store, "puzzle_id_target_api_version_major")?
		.call(&mut store, ())?;
	let api_version_minor: u32 = instance
		.get_typed_func(&mut store, "puzzle_id_target_api_version_minor")?
		.call(&mut store, ())?;
	let api_version_patch: u32 = instance
		.get_typed_func(&mut store, "puzzle_id_target_api_version_patch")?
		.call(&mut store, ())?;
	let api_version = (api_version_major, api_version_minor, api_version_patch);

	Ok(App {
		name: name.into(),
		version,
		api_version,
	})
}

pub fn link<T>(linker: &mut wasmtime::Linker<T>) -> wasmtime::Result<()> {
	linker.func_wrap("env", "puzzle_id_api_version_major", |_: Caller<'_, T>| {
		puzzle_common::API_VERSION.0
	})?;
	linker.func_wrap("env", "puzzle_id_api_version_minor", |_: Caller<'_, T>| {
		puzzle_common::API_VERSION.1
	})?;
	linker.func_wrap("env", "puzzle_id_api_version_patch", |_: Caller<'_, T>| {
		puzzle_common::API_VERSION.2
	})?;

	Ok(())
}
