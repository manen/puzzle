pub(crate) mod prelude {
	pub use super::IntoSocketOr;
}

#[derive(Debug, Clone)]
pub enum SocketOr<A: crate::Socket, B: crate::Socket> {
	A(A),
	B(B),
}
impl<A: crate::Socket, B: crate::Socket> Socket for SocketOr<A, B> {
	fn read(&mut self, buf: &mut [u8]) -> impl Future<Output = crate::Result<u32>> {
		async move {
			match self {
				SocketOr::A(a) => a.read(buf).await,
				SocketOr::B(b) => b.read(buf).await,
			}
		}
	}
	fn size_hint(&mut self) -> impl Future<Output = Option<u32>> {
		async move {
			match self {
				SocketOr::A(a) => a.size_hint().await,
				SocketOr::B(b) => b.size_hint().await,
			}
		}
	}
	fn write(&mut self, buf: &[u8]) -> impl Future<Output = crate::Result<u32>> {
		async move {
			match self {
				SocketOr::A(a) => a.write(buf).await,
				SocketOr::B(b) => b.write(buf).await,
			}
		}
	}
}

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
use std::future::Future;
use thiserror::Error;

use crate::Socket;

#[derive(Error, Debug, Clone)]
pub enum ErrorOr<A: error::Error + Debug, B: error::Error + Debug> {
	#[error("{0}")]
	A(A),
	#[error("{0}")]
	B(B),
}
impl<A: error::Error + Debug + From<crate::Error>, B: error::Error + Debug> From<crate::Error>
	for ErrorOr<A, B>
{
	fn from(value: crate::Error) -> Self {
		ErrorOr::A(value.into())
	}
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
