use std::iter;

use crate::{error::Operation, Error, Fs, Result, Socket};

#[derive(Default, Clone, Copy, Debug)]
pub struct EmptyFs;
impl Fs for EmptyFs {
	type ReadDir = iter::Empty<String>;
	type Socket = EmptySocket;

	async fn read_dir(&self, path: &str) -> Result<Self::ReadDir> {
		if path == "/" || path == "" {
			Ok(iter::empty())
		} else {
			Err(Error::NotFound {
				path_abs: path.to_owned(),
			})
		}
	}
	async fn open(&self, path: &str) -> Result<Self::Socket> {
		if path == "/" || path == "" {
			Err(crate::Error::DirOpen {
				path_abs: path.to_owned(),
			})
		} else {
			Err(crate::Error::NotFound {
				path_abs: path.to_owned(),
			})
		}
	}
}

#[derive(Copy, Clone, Debug)]
pub struct EmptySocket;
impl Socket for EmptySocket {
	fn read(&mut self, _: &mut [u8]) -> impl std::future::Future<Output = Result<u32>> {
		async {
			Err(Error::Empty {
				op: Operation::Read,
			})
		}
	}
	fn size_hint(&mut self) -> impl std::future::Future<Output = Option<u32>> {
		async { None }
	}
	fn write(&mut self, _: &[u8]) -> impl std::future::Future<Output = Result<u32>> {
		async {
			Err(Error::Empty {
				op: Operation::Write,
			})
		}
	}
}

pub fn empty() -> EmptyFs {
	EmptyFs
}
