use crate::{Error, Result};
use std::{future::Future, io::Read};

/// socket should be deinit on drop, up for the implementation to.. implement
///
/// socket details: there is no write and read universal specification, every file/socket gets to decide what to do
/// with its own writes and reads, they might append to a file, they might be sent over a network, they might be decoded and set as a variable for something who knows
pub trait Socket {
	fn read(&mut self, buf: &mut [u8]) -> impl Future<Output = Result<u32>>;
	fn size_hint(&mut self) -> impl Future<Output = Option<u32>>;

	fn write(&mut self, buf: &[u8]) -> impl Future<Output = Result<u32>>;
}

pub trait IntoSocket {
	type Socket: Socket;
	fn into_socket(self) -> Self::Socket;
}
impl<S: Socket> IntoSocket for S {
	type Socket = S;
	fn into_socket(self) -> Self::Socket {
		self
	}
}
impl IntoSocket for &'static str {
	type Socket = StaticStrSocket;
	fn into_socket(self) -> Self::Socket {
		StaticStrSocket { s: self }
	}
}

#[derive(Copy, Clone, Debug)]
pub struct StaticStrSocket {
	s: &'static str,
}
impl Socket for StaticStrSocket {
	fn read(&mut self, buf: &mut [u8]) -> impl Future<Output = Result<u32>> {
		async { Ok(self.s.as_bytes().read(buf)? as u32) }
	}
	fn size_hint(&mut self) -> impl Future<Output = Option<u32>> {
		async { Some(self.s.len() as u32) }
	}
	fn write(&mut self, _: &[u8]) -> impl Future<Output = Result<u32>> {
		async { Err(Error::ReadOnly) }
	}
}

// fix pls
