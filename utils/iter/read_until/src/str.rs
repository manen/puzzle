use crate::{IntoReader, Read, Reader};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StrReader<'a> {
	pub s: &'a str, // TODO rewrite this to use str.chars()
	pub i: usize,
}
impl<'a> Reader<'a> for StrReader<'a> {
	type Item = u8;
	type Output = &'a str;

	fn read_until<F: FnMut(&u8) -> bool>(&mut self, mut f: F) -> Read<Self::Output> {
		if self.i > self.s.len() {
			return Read::Finished;
		}

		let start_i = self.i;
		for item in &mut self.s[start_i..].bytes() {
			self.i += 1;
			if f(&item) {
				return Read::Condition(&self.s[start_i..self.i - 1]);
			}
		}
		Read::End(&self.s[start_i..self.i])
	}
}

impl IntoReader for str {
	#[allow(refining_impl_trait)]
	fn reader<'a>(&'a self) -> StrReader<'a> {
		StrReader { s: self, i: 0 }
	}
}
