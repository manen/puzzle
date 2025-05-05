use std::{
	borrow::Cow,
	fmt::{Debug, Display},
	io,
};
use thiserror::Error;

pub mod prelude {
	pub use super::Propagate;
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Error)]
pub enum Error {
	#[error("io err: {err}")]
	IoErr {
		#[from]
		err: crate::io_err::IoErr,
	},
	#[error("{original} (propagated from: {path})")]
	Propagated { original: Box<Error>, path: String },
	#[error("file or directory not found: {path_abs}")]
	NotFound { path_abs: String },
	#[error("attempted to open {path_abs} but it is a directory")]
	DirOpen { path_abs: String },
	#[error("not an absolute path: {path}")]
	NotAbs { path: String },
	#[error("error while {op}: empty socket")]
	Empty { op: Operation },
	#[error("attempted to write to readonly sock")]
	ReadOnly,
}
impl From<io::Error> for Error {
	fn from(value: io::Error) -> Self {
		let io_err: crate::io_err::IoErr = value.into();
		io_err.into()
	}
}
pub type Result<T> = std::result::Result<T, Error>;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub enum Operation {
	Read,
	Write,
}
impl Display for Operation {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// bit of an unorthodox implementation
		match self {
			Operation::Read => write!(f, "trying to read"),
			Operation::Write => write!(f, "trying to write"),
		}
	}
}

pub trait Propagate: Sized {
	fn propagate<'a>(self, path: impl Into<Cow<'a, str>>) -> Self;
}
impl Propagate for Error {
	fn propagate<'a>(self, path: impl Into<Cow<'a, str>>) -> Self {
		let path = path.into();
		match self {
			Error::Propagated {
				path: prop_path,
				original,
			} => Error::Propagated {
				original,
				path: format!("{path}{prop_path}"),
			},
			err => Error::Propagated {
				original: Box::new(err),
				path: path.to_string(),
			},
		}
	}
}
impl<T, E: Propagate> Propagate for std::result::Result<T, E> {
	fn propagate<'a>(self, path: impl Into<Cow<'a, str>>) -> Self {
		self.map_err(|err| err.propagate(path))
	}
}

pub fn abs_check<'a, S: Into<Cow<'a, str>>>(path: S) -> Result<()> {
	let path = path.into();
	if path.starts_with("/") || path == "" {
		Ok(())
	} else {
		Err(Error::NotAbs {
			path: path.to_string(),
		})
	}
}
