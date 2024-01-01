#[cfg(test)]
mod tests;

mod slice;
mod str;
pub use slice::*;
pub use str::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Read<T> {
	Finished,
	Condition(T),
	End(T),
}

pub trait IntoReader<'a> {
	type Reader: Reader<'a>;

	fn reader(&'a self) -> Self::Reader;
}
pub trait Reader<'a> {
	type Item;
	type Output;

	fn read_until<F: Fn(Self::Item) -> bool>(&mut self, f: F) -> Read<Self::Output>;
}
