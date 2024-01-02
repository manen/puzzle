#[cfg(test)]
mod tests;

mod iter;
mod slice;
mod str;
pub use iter::*;
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

	fn read_until<F: Fn(&Self::Item) -> bool>(&mut self, f: F) -> Read<Self::Output>;

	fn read_until_item<E: PartialEq<Self::Item>>(&mut self, item: E) -> Read<Self::Output> {
		self.read_until(|a| item.eq(a))
	}
}
