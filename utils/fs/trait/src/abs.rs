use crate::Result;
use std::borrow::Cow;

pub fn remove_tail<'a, P: Into<Cow<'a, str>>>(path: P) -> Cow<'a, str> {
	let path = path.into();
	if !path.ends_with('/') {
		path
	} else {
		match path {
			Cow::Borrowed(slice) => Cow::Borrowed(&slice[..slice.len() - 1]),
			Cow::Owned(s) => Cow::Owned(String::from(&s[..s.len() - 1])),
		}
	}
}
pub fn add_tail<'a, P: Into<Cow<'a, str>>>(path: P) -> Cow<'a, str> {
	let path = path.into();
	if path.ends_with('/') {
		path
	} else {
		Cow::Owned(format!("{path}/"))
	}
}

pub fn absify<'a, P: Into<Cow<'a, str>>>(path: P) -> Cow<'a, str> {
	let path = path.into();
	if path.starts_with('/') {
		path
	} else {
		Cow::Owned(format!("/{path}"))
	}
}

#[derive(Copy, Clone, Debug)]
pub struct Abs<F: crate::Fs> {
	pub(crate) fs: F,
}
impl<F: crate::Fs> crate::Fs for Abs<F> {
	type ReadDir = F::ReadDir;
	type Socket = F::Socket;

	fn read_dir(&self, path: &str) -> Result<Self::ReadDir> {
		self.fs.read_dir(&remove_tail(absify(path)))
	}
	fn open(&self, path: &str) -> Result<Self::Socket> {
		self.fs.open(&remove_tail(absify(path)))
	}
}
