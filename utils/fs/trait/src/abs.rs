use crate::Result;
use std::{borrow::Cow, future::Future};

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
impl<F: crate::Fs + Send + Sync> crate::Fs for Abs<F> {
	type ReadDir = F::ReadDir;
	type Socket = F::Socket;

	fn read_dir(&self, path: &str) -> impl Future<Output = Result<Self::ReadDir>> + Send {
		let abs = absify(path);
		async { self.fs.read_dir(&remove_tail(abs)).await }
	}
	fn open(&self, path: &str) -> impl Future<Output = Result<Self::Socket>> + Send {
		let abs = absify(path);
		async { self.fs.open(&remove_tail(abs)).await }
	}
}
