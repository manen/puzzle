use anyhow::{anyhow, Result};

#[cfg(test)]
mod tests;

pub trait Engine: Default {
	type Linker: Linker;

	fn linker(&self, bin: &[u8]) -> Result<Self::Linker>;
}
pub trait Linker {
	type Instance: Instance;
	type Args: Args;

	fn link<O: Arg, F: Fn(Self::Args) -> O + Send + Sync + 'static>(
		&mut self,
		name: &str,
		f: F,
	) -> Result<()>;
	fn start(self) -> Result<Self::Instance>;
}
pub trait Instance {
	fn call<I: Args, O: Arg>(&mut self, func: &str, args: I) -> Result<O>;
}

pub trait Args {
	fn len(&self) -> usize;
	fn args(self) -> impl Iterator<Item = ArgDyn>;
}
impl Args for () {
	fn len(&self) -> usize {
		0
	}
	fn args(self) -> impl Iterator<Item = ArgDyn> {
		std::iter::empty()
	}
}
impl<A: Arg, const LEN: usize> Args for [A; LEN] {
	fn len(&self) -> usize {
		LEN
	}
	fn args(self) -> impl Iterator<Item = ArgDyn> {
		self.map(|arg| arg.arg_dyn()).into_iter()
	}
}
impl<T1: Arg> Args for T1 {
	fn len(&self) -> usize {
		1
	}
	fn args(self) -> impl Iterator<Item = ArgDyn> {
		[self.arg_dyn()].into_iter()
	}
}
impl<T1: Arg, T2: Arg> Args for (T1, T2) {
	fn len(&self) -> usize {
		2
	}
	fn args(self) -> impl Iterator<Item = ArgDyn> {
		[self.0.arg_dyn(), self.1.arg_dyn()].into_iter()
	}
}
impl<T1: Arg, T2: Arg, T3: Arg> Args for (T1, T2, T3) {
	fn len(&self) -> usize {
		3
	}
	fn args(self) -> impl Iterator<Item = ArgDyn> {
		[self.0.arg_dyn(), self.1.arg_dyn(), self.2.arg_dyn()].into_iter()
	}
}
impl<T1: Arg, T2: Arg, T3: Arg, T4: Arg> Args for (T1, T2, T3, T4) {
	fn len(&self) -> usize {
		4
	}
	fn args(self) -> impl Iterator<Item = ArgDyn> {
		[
			self.0.arg_dyn(),
			self.1.arg_dyn(),
			self.2.arg_dyn(),
			self.3.arg_dyn(),
		]
		.into_iter()
	}
}
impl<T1: Arg, T2: Arg, T3: Arg, T4: Arg, T5: Arg> Args for (T1, T2, T3, T4, T5) {
	fn len(&self) -> usize {
		5
	}
	fn args(self) -> impl Iterator<Item = ArgDyn> {
		[
			self.0.arg_dyn(),
			self.1.arg_dyn(),
			self.2.arg_dyn(),
			self.3.arg_dyn(),
			self.4.arg_dyn(),
		]
		.into_iter()
	}
}
// TODO add more if needed

pub struct IteratorArgs<I: Iterator<Item = ArgDyn>> {
	i: I,
}
impl<I: Iterator<Item = ArgDyn>> IteratorArgs<I> {
	pub fn new(i: I) -> Self {
		IteratorArgs { i }
	}
}
impl<I: Iterator<Item = ArgDyn>> Args for IteratorArgs<I> {
	fn len(&self) -> usize {
		self.i.size_hint().0
	}
	fn args(self) -> impl Iterator<Item = ArgDyn> {
		self.i
	}
}

pub trait Arg: Sized {
	fn from_arg_dyn(arg: ArgDyn) -> Result<Self>;
	fn arg_dyn(self) -> ArgDyn;
}
macro_rules! arg_impl {
	($ty:ty; $var:ident) => {
		impl Arg for $ty {
			fn from_arg_dyn(arg: ArgDyn) -> Result<Self> {
				match arg {
					ArgDyn::$var(a) => Ok(a),
					arg => Err(anyhow!(
						"failed to cast ArgDyn: expected {}, got {arg:?}",
						stringify!($ty)
					)),
				}
			}
			fn arg_dyn(self) -> ArgDyn {
				ArgDyn::$var(self)
			}
		}
	};
}

arg_impl!(i32; I32);
arg_impl!(i64; I64);
arg_impl!(f32; F32);
arg_impl!(f64; F64);
arg_impl!(u128; U128);

#[derive(Clone, Debug, PartialEq)]
pub enum ArgDyn {
	I32(i32),
	I64(i64),
	F32(f32),
	F64(f64),
	U128(u128),
}
impl ArgDyn {
	pub fn arg<I: Arg>(self) -> Result<I> {
		I::from_arg_dyn(self)
	}
}
impl Arg for ArgDyn {
	fn from_arg_dyn(arg: ArgDyn) -> Result<Self> {
		Ok(arg)
	}
	fn arg_dyn(self) -> ArgDyn {
		self
	}
}
