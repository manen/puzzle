use crate::{Read, Reader};

pub struct IteratorReader<I: Iterator> {
	iter: I,
}
impl<'a, I: Iterator> Reader<'a> for IteratorReader<I>
where
	Self: 'a,
{
	type Item = I::Item;
	type Output = Vec<I::Item>;

	fn read_until<F: Fn(&Self::Item) -> bool>(&mut self, f: F) -> Read<Self::Output> {
		let mut buf = Vec::new();

		for item in &mut self.iter {
			if f(&item) {
				return Read::Condition(buf);
			} else {
				buf.push(item);
			}
		}
		if buf.len() == 0 {
			Read::Finished
		} else {
			Read::End(buf)
		}
	}
}

pub trait IntoIteratorReader: Iterator + Sized {
	fn reader(self) -> IteratorReader<Self>;
}
impl<I: Iterator> IntoIteratorReader for I {
	fn reader(self) -> IteratorReader<Self> {
		IteratorReader { iter: self }
	}
}
