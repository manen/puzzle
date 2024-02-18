use anyhow::anyhow;
use std::vec::IntoIter;
use thiserror::Error;
use wasm_trait::Arg;
use wasmtime::{Engine, Module, Store};

#[derive(Default)]
pub struct WasmtimeEngine(Engine);
impl wasm_trait::Engine for WasmtimeEngine {
	type Linker = WasmtimeLinker;

	fn linker(&self, bin: &[u8]) -> anyhow::Result<Self::Linker> {
		let module = Module::from_binary(&self.0, bin)?;
		let store = Store::new(&self.0, Runtime::default());
		let linker = wasmtime::Linker::new(&self.0);
		Ok(WasmtimeLinker {
			module,
			store,
			linker,
		})
	}
}

pub struct WasmtimeLinker {
	module: Module,
	store: Store<Runtime>,
	linker: wasmtime::Linker<Runtime>,
}
impl wasm_trait::Linker for WasmtimeLinker {
	type Instance = WasmtimeInstance;
	type Args = wasm_trait::IteratorArgs<IntoIter<wasm_trait::ArgDyn>>;

	fn link<O: wasm_trait::Arg, F: Fn(Self::Args) -> O + Send + Sync + 'static>(
		&mut self,
		name: &str,
		f: F,
	) -> anyhow::Result<()> {
		self.linker.func_new(
			"env",
			name,
			wasmtime::FuncType::new(std::iter::empty(), std::iter::empty()),
			move |_: wasmtime::Caller<'_, Runtime>, i, o| {
				let args = i.into_iter().map(|val| match val {
					wasmtime::Val::I32(num) => Ok(wasm_trait::ArgDyn::I32(*num)),
					wasmtime::Val::I64(num) => Ok(wasm_trait::ArgDyn::I64(*num)),
					wasmtime::Val::F32(num) => Ok(wasm_trait::ArgDyn::F32(f32::from_bits(*num))),
					wasmtime::Val::F64(num) => Ok(wasm_trait::ArgDyn::F64(f64::from_bits(*num))),
					wasmtime::Val::V128(num) => Ok(wasm_trait::ArgDyn::U128((*num).into())),
					_ => Err(anyhow!(
						"funcrefs and externrefs can't be turned into an ArgDyn"
					)),
				});
				let args = args.collect::<Result<Vec<_>, _>>()?;
				let out = f(wasm_trait::IteratorArgs::new(args.into_iter()));
				o[0] = match out.arg_dyn() {
					wasm_trait::ArgDyn::I32(num) => wasmtime::Val::I32(num),
					wasm_trait::ArgDyn::I64(num) => wasmtime::Val::I64(num),
					wasm_trait::ArgDyn::F32(num) => wasmtime::Val::F32(num.to_bits()),
					wasm_trait::ArgDyn::F64(num) => wasmtime::Val::F64(num.to_bits()),
					wasm_trait::ArgDyn::U128(num) => wasmtime::Val::V128(num.into()),
				};
				Ok(())
			},
		)?;
		// match I::LEN {
		// 	0 => self.linker.func
		// };
		Ok(())
	}

	fn start(mut self) -> anyhow::Result<Self::Instance> {
		let instance = self.linker.instantiate(&mut self.store, &self.module)?;
		Ok(WasmtimeInstance {
			store: self.store,
			instance: instance,
		})
	}
}

pub struct WasmtimeInstance {
	store: Store<Runtime>,
	instance: wasmtime::Instance,
}
impl wasm_trait::Instance for WasmtimeInstance {
	fn call<I: wasm_trait::Args, O: wasm_trait::Arg>(
		&mut self,
		func: &str,
		args: I,
	) -> anyhow::Result<O> {
		let args = args
			.args()
			.map(|arg| match arg.arg_dyn() {
				wasm_trait::ArgDyn::I32(num) => wasmtime::Val::I32(num),
				wasm_trait::ArgDyn::I64(num) => wasmtime::Val::I64(num),
				wasm_trait::ArgDyn::F32(num) => wasmtime::Val::F32(num.to_bits()),
				wasm_trait::ArgDyn::F64(num) => wasmtime::Val::F64(num.to_bits()),
				wasm_trait::ArgDyn::U128(num) => wasmtime::Val::V128(num.into()),
			})
			.collect::<Vec<_>>();
		let func = self
			.instance
			.get_func(&mut self.store, func)
			.ok_or_else(|| anyhow!("function {func} does not exist"))?;
		let mut out = [wasmtime::Val::ExternRef(None)]; // this is so it doesnt cry cause its empty
		func.call(&mut self.store, &args, &mut out)?;

		Ok(match out[0] {
			wasmtime::Val::I32(num) => Ok(wasm_trait::ArgDyn::I32(num)),
			wasmtime::Val::I64(num) => Ok(wasm_trait::ArgDyn::I64(num)),
			wasmtime::Val::F32(num) => Ok(wasm_trait::ArgDyn::F32(f32::from_bits(num))),
			wasmtime::Val::F64(num) => Ok(wasm_trait::ArgDyn::F64(f64::from_bits(num))),
			wasmtime::Val::V128(num) => Ok(wasm_trait::ArgDyn::U128(num.into())),
			_ => Err(anyhow!(
				"funcrefs and externrefs can't be turned into an ArgDyn"
			)),
		}?
		.arg()?)
	}
}

#[derive(Debug, Error)]
pub enum Error {
	#[error("wasmtime error: {0}")]
	Wasmtime(#[from] anyhow::Error),
	#[error("id error: {0}")]
	Id(#[from] id::RtError),
}
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Default)]
pub struct Runtime {}

// pub fn start(wasm: &[u8]) -> Result<()> {
// 	let engine = Engine::default();
// 	let module = Module::from_binary(&engine, wasm)?;

// 	let mut store = Store::new(&engine, Runtime::default());

// 	let mut linker = Linker::new(&engine);
// 	puzzle_log::link(&mut linker)?;
// 	id::link(&mut linker)?;

// 	let instance = linker.instantiate(&mut store, &module)?;

// 	let puzzle_main = instance.get_typed_func::<(), ()>(&mut store, "puzzle_main")?;
// 	puzzle_main.call(&mut store, ())?;
// 	id::ensure_app_api_version(&mut store, &instance)?;

// 	let app = id::app(&mut store, &instance)?;
// 	log::info!("running app: {app}");

// 	// let puzzle_render = instance.get_typed_func::<(), ()>(&mut store, "puzzle_render")?;

// 	Ok(())
// }
