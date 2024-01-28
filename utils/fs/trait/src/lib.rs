use std::io::{Read, Write};

pub mod abs;
pub mod empty;
pub mod error;
pub mod file_mount;
pub mod fs_mount;
pub mod or;
pub mod quicksocket;
#[cfg(test)]
mod tests;

pub mod prelude {
	pub use crate::error::prelude::*;
	pub use crate::or::prelude::*;
	pub use crate::quicksocket::prelude::*;
	pub use crate::Fs;
}

use abs::Abs;
pub use empty::{empty, EmptyFs};
pub use error::{Error, Result};
pub use file_mount::FileMount;
use fs_mount::FsMount;
pub use or::IntoSocketOr;

/// this trait is very much like `Iterator`, it defines some functions necessary for filesystem functions,
/// and defines functions for modifying the current `Fs` fully functionally and at compile-time
pub trait Fs: Sized {
	type ReadDir: Iterator<Item = String>;
	type Socket: Socket;

	/// read_dir returns an iterator over absolute paths of items in the directory
	fn read_dir(&self, path: &str) -> Result<Self::ReadDir>;
	/// open opens a socket to a given path
	fn open(&self, path: &str) -> Result<Self::Socket>;

	// - modifier functions
	/// mount a file on top of this filesystem
	fn mount_file<P: Into<String>, S: Socket>(self, path: P, socket: S) -> FileMount<Self, S> {
		let path = path.into();
		FileMount {
			fs: self,
			path,
			socket,
		}
	}
	/// mounts a subfilesystem (aka directory) to a path
	fn mount_fs<P: Into<String>, F: Fs>(self, path: P, fs: F) -> FsMount<Self, F> {
		let path = path.into();
		FsMount {
			a: self,
			b: fs,
			path,
		}
	}
	/// absifies all requests to this filesystem
	fn abs(self) -> Abs<Self> {
		Abs { fs: self }
	}
}

/// socket should be deinit on drop, up for the implementation to.. implement
///
/// socket details: there is no write and read universal specification, every file/socket gets to decide what to do
/// with its own writes and reads, they might append to a file, they might be sent over a network, they might be decoded and set as a variable for something who knows
pub trait Socket: Write + Read {}
