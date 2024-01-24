use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Shared {
	Ping,
	PingAck,
}
impl Shared {
	pub fn handle<T: From<Shared>>(&self) -> Option<T> {
		match self {
			Shared::Ping => Some(Shared::PingAck.into()),
			Shared::PingAck => None,
		}
	}
}
