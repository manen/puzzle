use std::io;

pub mod prelude {
	pub use super::IntoSocketOr;
}

#[derive(Debug, Clone)]
pub enum SocketOr<A: crate::Socket, B: crate::Socket> {
	A(A),
	B(B),
}
impl<A: crate::Socket, B: crate::Socket> io::Write for SocketOr<A, B> {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		match self {
			SocketOr::A(a) => a.write(buf),
			SocketOr::B(b) => b.write(buf),
		}
	}
	fn flush(&mut self) -> io::Result<()> {
		match self {
			SocketOr::A(a) => a.flush(),
			SocketOr::B(b) => b.flush(),
		}
	}
}
impl<A: crate::Socket, B: crate::Socket> io::Read for SocketOr<A, B> {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		match self {
			SocketOr::A(a) => a.read(buf),
			SocketOr::B(b) => b.read(buf),
		}
	}
}
impl<A: crate::Socket, B: crate::Socket> crate::Socket for SocketOr<A, B> {}

pub trait IntoSocketOr: crate::Socket + Sized {
	fn a<B: crate::Socket>(self) -> SocketOr<Self, B> {
		SocketOr::A(self)
	}
	fn b<A: crate::Socket>(self) -> SocketOr<A, Self> {
		SocketOr::B(self)
	}
}
impl<S: crate::Socket> IntoSocketOr for S {}

use std::error;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ErrorOr<A: error::Error + Debug, B: error::Error + Debug> {
	#[error("{0}")]
	A(A),
	#[error("{0}")]
	B(B),
}
#[derive(Debug, Clone)]
pub enum IteratorOr<T, A: Iterator<Item = T>, B: Iterator<Item = T>> {
	A(A),
	B(B),
}
impl<T, A: Iterator<Item = T>, B: Iterator<Item = T>> Iterator for IteratorOr<T, A, B> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		match self {
			IteratorOr::A(a) => a.next(),
			IteratorOr::B(b) => b.next(),
		}
	}
	fn size_hint(&self) -> (usize, Option<usize>) {
		match self {
			IteratorOr::A(a) => a.size_hint(),
			IteratorOr::B(b) => b.size_hint(),
		}
	}
}
