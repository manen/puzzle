use std::future::Future;

use crate::{or, prelude::*, IntoSocketOr, Result};

#[derive(Clone, Debug)]
pub enum ReadDir<A: Iterator<Item = String>, B: Iterator<Item = String>> {
	A { iter: A, path: Option<String> },
	B { iter: B, path: String },
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
			ReadDir::B { iter, path } => iter.next().map(|short| format!("{path}{short}")),
		}
	}
	fn size_hint(&self) -> (usize, Option<usize>) {
		match self {
			ReadDir::A { iter, path } => {
				let size_hint = iter.size_hint();
				let add = if path.is_some() { 1 } else { 0 };
				(size_hint.0 + add, size_hint.1.map(|x| x + add))
			}
			ReadDir::B { iter, .. } => iter.size_hint(),
		}
	}
}

#[derive(Clone, Debug)]
pub struct FsMount<A: crate::Fs, B: crate::Fs> {
	pub(crate) a: A,
	pub(crate) path: String,
	pub(crate) b: B,
}
impl<A: crate::Fs, B: crate::Fs> crate::Fs for FsMount<A, B> {
	type ReadDir = ReadDir<A::ReadDir, B::ReadDir>;
	type Socket = or::SocketOr<A::Socket, B::Socket>;

	fn read_dir(&self, path: &str) -> impl Future<Output = Result<Self::ReadDir>> {
		async move {
			crate::error::abs_check(path)?;
			if path.starts_with(&self.path) {
				Ok(ReadDir::B {
					iter: self
						.b
						.read_dir(&path.replacen(&self.path, "", 1))
						.await
						.propagate(&self.path)?,
					path: self.path.clone(),
				})
			} else {
				Ok(ReadDir::A {
					iter: self.a.read_dir(path).await.propagate(&self.path)?,
					path: Some(self.path.clone()),
				})
			}
		}
	}
	fn open(&self, path: &str) -> impl Future<Output = Result<Self::Socket>> {
		async move {
			crate::error::abs_check(path)?;
			if path.starts_with(&self.path) {
				Ok(self.b.open(&path.replacen(&self.path, "", 1)).await?.b())
			} else {
				Ok(self.a.open(path).await?.a())
			}
		}
	}
}
