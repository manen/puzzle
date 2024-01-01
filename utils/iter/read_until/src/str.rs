use crate::{IntoReader, Read, Reader};

pub struct StrReader<'a> {
	s: &'a str,
	i: usize,
}
impl<'a> Reader<'a> for StrReader<'a> {
	type Item = u8;
	type Output = &'a str;

	fn read_until<F: Fn(u8) -> bool>(&mut self, f: F) -> Read<Self::Output> {
		if self.i > self.s.len() {
			return Read::Finished;
		}

		let start_i = self.i;
		for item in &mut self.s[start_i..].bytes() {
			self.i += 1;
			if f(item) {
				return Read::Condition(&self.s[start_i..self.i - 1]);
			}
		}
		Read::End(&self.s[start_i..self.i])
	}
}

impl<'a> IntoReader<'a> for &'a str {
	type Reader = StrReader<'a>;

	fn reader(&'a self) -> Self::Reader {
		StrReader { s: self, i: 0 }
	}
}
