use crate::{IntoReader, Read, Reader};

pub struct SliceReader<'a, T> {
	slice: &'a [T],
	i: usize,
}
impl<'a, T> Reader<'a> for SliceReader<'a, T> {
	type Item = T;
	type Output = &'a [T];

	fn read_until<F: FnMut(&T) -> bool>(&mut self, mut f: F) -> Read<&'a [T]> {
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

impl<T> IntoReader for [T] {
	#[allow(refining_impl_trait)]
	fn reader<'a>(&'a self) -> SliceReader<'a, T> {
		SliceReader {
			slice: self.as_ref(),
			i: 0,
		}
	}
}
