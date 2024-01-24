use crate::Shared;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ToServer {
	Shared(Shared),
}
impl From<Shared> for ToServer {
	fn from(value: Shared) -> Self {
		ToServer::Shared(value)
	}
}
