use crate::or::{self, IntoSocketOr};
use std::{error, fmt::Debug, iter};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error<E: error::Error + Debug> {
	#[error("{0}")]
	Fs(E),
}

pub struct FileMount<Fs: crate::Fs, S: crate::Socket> {
	pub(crate) fs: Fs,
	pub(crate) path: String,
	pub(crate) socket: S,
}
impl<Fs, S> crate::Fs for FileMount<Fs, S>
where
	Fs: crate::Fs,
	S: crate::Socket + Clone,
{
	type ReadDir = iter::Chain<Fs::ReadDir, iter::Once<String>>;
	type Socket = or::SocketOr<Fs::Socket, S>;
	type Error = Error<Fs::Error>;

	fn read_dir(&self, path: &str) -> Result<Self::ReadDir, Self::Error> {
		Ok(self
			.fs
			.read_dir(path)
			.map_err(|err| Error::Fs(err))?
			.chain(iter::once(self.path.clone())))
	}
	fn open(&self, path: &str) -> Result<Self::Socket, Self::Error> {
		if self.path == path {
			Ok(self.socket.clone().b())
		} else {
			self.fs
				.open(path)
				.map_err(|err| Error::Fs(err))
				.map(|socket| socket.a())
		}
	}
}
