use std::{io, iter};

use crate::{Error, Fs, Result};

#[derive(Default, Clone, Copy, Debug)]
pub struct EmptyFs;
impl Fs for EmptyFs {
	type ReadDir = iter::Empty<String>;
	type Socket = EmptySocket;

	fn read_dir(&self, path: &str) -> Result<Self::ReadDir> {
		if path == "/" || path == "" {
			Ok(iter::empty())
		} else {
			Err(Error::NotFound {
				path_abs: path.to_owned(),
			})
		}
	}
	fn open(&self, path: &str) -> Result<Self::Socket> {
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

pub fn empty() -> EmptyFs {
	EmptyFs
}
