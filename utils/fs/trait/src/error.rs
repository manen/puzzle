use std::{borrow::Cow, fmt::Debug};
use thiserror::Error;

pub mod prelude {
	pub use super::Propagate;
}

#[derive(Clone, Debug, Error)]
pub enum Error {
	#[error("{err} (propagated from: {original})")]
	Propagated { err: Box<Error>, original: String },
	#[error("file or directory not found: {path_abs}")]
	NotFound { path_abs: String },
	#[error("attempted to open {path_abs} but it is a directory")]
	DirOpen { path_abs: String },
	#[error("not an absolute path: {path}")]
	NotAbs { path: String },
}
pub type Result<T> = std::result::Result<T, Error>;

pub trait Propagate: Sized {
	fn propagate(self, path: &str) -> Self;
}
impl Propagate for Error {
	fn propagate(self, path: &str) -> Self {
		match self {
			Error::Propagated { err, original } => Error::Propagated {
				err: Box::new(err.propagate(path)),
				original,
			},
			Error::NotFound { path_abs: original } => Error::Propagated {
				err: Box::new(Error::NotFound {
					path_abs: format!("{path}{original}"),
				}),
				original,
			},
			Error::DirOpen { path_abs: original } => Error::Propagated {
				err: Box::new(Error::DirOpen {
					path_abs: format!("{path}{original}"),
				}),
				original,
			},
			Error::NotAbs { path: original } => Error::Propagated {
				err: Box::new(Error::NotAbs {
					path: format!("{path}{original}"),
				}),
				original,
			},
		}
	}
}
impl<T, E: Propagate> Propagate for std::result::Result<T, E> {
	fn propagate(self, path: &str) -> Self {
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
