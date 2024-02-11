use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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
		dir: fs_trait::Result<Cow<'a, [String]>>,
	},
	Open {
		sock: fs_trait::Result<RemoteSocketID>,
	},
	Read {
		sock: RemoteSocketID,
		buf: io_err::Result<Cow<'a, [u8]>>,
	},
	Write {
		sock: RemoteSocketID,
		len: io_err::Result<u32>,
	},
}

pub type RemoteSocketID = u64;
