use crate::{IntoReader, Read, Reader};

pub struct SliceReader<'a, T> {
	slice: &'a [T],
	i: usize,
}
impl<'a, T> Reader<'a> for SliceReader<'a, T> {
	type Item = &'a T;
	type Output = &'a [T];

	fn read_until<F: Fn(&'a T) -> bool>(&mut self, f: F) -> Read<&'a [T]> {
		if self.i >= self.slice.len() {
			return Read::Finished;
		}

		let start_i = self.i;
		for item in &self.slice[start_i..] {
			self.i += 1;
			if f(item) {
				return Read::Condition(&self.slice[start_i..self.i - 1]);
			}
		}
		Read::End(&self.slice[start_i..self.i])
	}
}

impl<'a, T> IntoReader<'a> for &'a [T] {
	type Reader = SliceReader<'a, T>;

	fn reader(&'a self) -> SliceReader<'a, T> {
		SliceReader {
			slice: self.as_ref(),
			i: 0,
		}
	}
}
