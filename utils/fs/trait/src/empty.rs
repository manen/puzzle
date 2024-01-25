use std::{io, iter};

use thiserror::Error;

use crate::Fs;

#[derive(Error, Debug, Clone)]
pub enum Error {
	#[error("file or directory not found: {path}")]
	NotFound { path: String },
}

#[derive(Default, Clone, Copy, Debug)]
pub struct EmptyFs;
impl Fs for EmptyFs {
	type ReadDir = iter::Empty<String>;
	type Error = Error;
	type Socket = EmptySocket;

	fn read_dir(&self, path: &str) -> Result<Self::ReadDir, Self::Error> {
		Err(Error::NotFound {
			path: path.to_owned(),
		})
	}
	fn open(&self, path: &str) -> Result<Self::Socket, Self::Error> {
		Err(Error::NotFound {
			path: path.to_owned(),
		})
	}
}

pub struct EmptySocket;
impl io::Write for EmptySocket {
	fn write(&mut self, _: &[u8]) -> io::Result<usize> {
		Err(io::Error::new(
			io::ErrorKind::Unsupported,
			"emptysocket shouldn't be written to",
		))
	}
	fn flush(&mut self) -> io::Result<()> {
		Err(io::Error::new(
			io::ErrorKind::Unsupported,
			"emptysocket shouldn't be written to",
		))
	}
}
impl io::Read for EmptySocket {
	fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
		Err(io::Error::new(
			io::ErrorKind::Unsupported,
			"emptysocket shouldn't be read from",
		))
	}
}
impl crate::Socket for EmptySocket {}

pub fn empty() -> EmptyFs {
	EmptyFs::default()
}
