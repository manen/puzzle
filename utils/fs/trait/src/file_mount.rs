use crate::{
	or::{self, IntoSocketOr},
	Result,
};
use std::{fmt::Debug, iter};

#[derive(Clone, Debug)]
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

	fn read_dir(&self, path: &str) -> Result<Self::ReadDir> {
		crate::error::abs_check(path)?;
		Ok(self.fs.read_dir(path)?.chain(iter::once(self.path.clone())))
	}
	fn open(&self, path: &str) -> Result<Self::Socket> {
		crate::error::abs_check(path)?;
		if self.path == path {
			Ok(self.socket.clone().b())
		} else {
			Ok(self.fs.open(path)?.a())
		}
	}
}
