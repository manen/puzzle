use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt::Display};
use thiserror::Error;

pub mod io_err;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Request<'a> {
	ReadDir {
		path: Cow<'a, str>,
	},
	Open {
		path: Cow<'a, str>,
	},
	Read {
		sock: RemoteSocketID,
		len: u32,
	},
	Write {
		sock: RemoteSocketID,
		buf: Cow<'a, [u8]>,
	},
	Close {
		sock: RemoteSocketID,
	},
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Response<'a> {
	ReadDir {
		dir: fs_trait::Result<Vec<String>>,
	},
	Open {
		sock: fs_trait::Result<RemoteSocketID>,
	},
	Read {
		sock: RemoteSocketID,
		buf: Result<Cow<'a, [u8]>>,
	},
	Write {
		sock: RemoteSocketID,
		len: Result<u32>,
	},
}

#[derive(Clone, Debug, Error, Serialize, Deserialize)]
pub enum Error {
	#[error("{0}")]
	IoErr(#[from] io_err::IoErr),
	#[error("invalid socket {sock} while {op}")]
	InvalidSock { sock: RemoteSocketID, op: Operation },
}
#[derive(Clone, Debug, Serialize, Deserialize)]
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
pub type Result<T> = std::result::Result<T, Error>;

pub type RemoteSocketID = u64;
