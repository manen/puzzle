#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Read<'a, T> {
	Finished,
	Condition(&'a [T]),
	End(&'a [T]),
}

pub struct ReadUntil<'a, T> {
	slice: &'a [T],
	i: usize,
}
impl<'a, T> ReadUntil<'a, T> {
	pub fn read_until<F: Fn(&T) -> bool>(&mut self, f: F) -> Read<'a, T> {
		if self.i >= self.slice.len() {
			return Read::Finished;
		}

		let start_i = self.i;
		for item in &self.slice[self.i..] {
			self.i += 1;
			if f(item) {
				return Read::Condition(&self.slice[start_i..self.i - 1]);
			}
		}
		Read::End(&self.slice[start_i..self.i])
	}
}

pub trait IntoReadUntil<T> {
	fn read_until<'a>(&'a self) -> ReadUntil<'a, T>;
}
impl<T, A: AsRef<[T]>> IntoReadUntil<T> for A {
	fn read_until<'a>(&'a self) -> ReadUntil<'a, T> {
		ReadUntil {
			slice: self.as_ref(),
			i: 0,
		}
	}
}
