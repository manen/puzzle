use std::io::{Read, Write};

#[cfg(test)]
mod tests;

/// this trait is very much like `Iterator`, it defines some functions necessary for filesystem functions,
/// and defines functions for modifying the current `Fs` fully functionally and at compile-time
pub trait Fs {
	type Result<T>: std::error::Error;
	type ReadDir: Iterator<Item = String>;
	type Socket: Socket;

	/// read_dir returns an iterator over absolute paths of items in the directory
	fn read_dir(&self, path: &str) -> Self::Result<Self::ReadDir>;
	/// open opens a socket to a given path
	fn open(&self, path: &str) -> Self::Result<Self::Socket>;

	// - modifier functions
}

/// socket should be deinit on drop, up for the implementation to.. implement
///
/// socket details: there is no write and read universal specification, every file/socket gets to decide what to do
/// with its own writes and reads, they might append to a file, they might be sent over a network, they might be decoded and set as a variable for something who knows
pub trait Socket: Write + Read {}
