#[macro_export]
macro_rules! glue {
	($runtime_mod:ident,$wasm_mod:ident) => {
		#[cfg(feature = "runtime")]
		pub use $runtime_mod::*;
		#[cfg(feature = "wasm")]
		pub use $wasm_mod::*;
	};
}
