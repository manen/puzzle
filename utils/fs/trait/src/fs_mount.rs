use crate::{or, IntoSocketOr};

#[derive(Clone, Debug)]
pub enum ReadDir<A: Iterator<Item = String>, B: Iterator<Item = String>> {
	A { iter: A, path: Option<String> },
	B(B),
}
impl<A: Iterator<Item = String>, B: Iterator<Item = String>> Iterator for ReadDir<A, B> {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		match self {
			ReadDir::A { iter, path } => match iter.next() {
				Some(a) => Some(a),
				None => match path.take() {
					Some(a) => Some(a),
					None => None,
				},
			},
			ReadDir::B(iter) => iter.next(),
		}
	}
	fn size_hint(&self) -> (usize, Option<usize>) {
		match self {
			ReadDir::A { iter, path } => {
				let size_hint = iter.size_hint();
				let add = if path.is_some() { 1 } else { 0 };
				(size_hint.0 + add, size_hint.1.map(|x| x + add))
			}
			ReadDir::B(iter) => iter.size_hint(),
		}
	}
}

pub struct FsMount<A: crate::Fs, B: crate::Fs> {
	pub(crate) a: A,
	pub(crate) path: String,
	pub(crate) b: B,
}
impl<A: crate::Fs, B: crate::Fs> crate::Fs for FsMount<A, B> {
	type ReadDir = ReadDir<A::ReadDir, B::ReadDir>;
	type Socket = or::SocketOr<A::Socket, B::Socket>;
	type Error = or::ErrorOr<A::Error, B::Error>;

	fn read_dir(&self, path: &str) -> Result<Self::ReadDir, Self::Error> {
		if path.starts_with(&self.path) {
			self.b
				.read_dir(&path.replacen(&self.path, "", 1))
				.map(|readdir| ReadDir::B(readdir))
				.map_err(|err| or::ErrorOr::B(err))
		} else {
			self.a
				.read_dir(path)
				.map(|readdir| ReadDir::A {
					iter: readdir,
					path: Some(crate::abs::remove_tail(&self.path).to_string()),
				})
				.map_err(|err| or::ErrorOr::A(err))
		}
	}
	fn open(&self, path: &str) -> Result<Self::Socket, Self::Error> {
		if path.starts_with(&self.path) {
			self.b
				.open(&path.replacen(&self.path, "", 1))
				.map(|socket| socket.b())
				.map_err(|err| or::ErrorOr::B(err))
		} else {
			self.a
				.open(path)
				.map(|socket| socket.a())
				.map_err(|err| or::ErrorOr::A(err))
		}
	}
}
