use crate::Shared;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ToClient {
	Shared(Shared),
}
impl From<Shared> for ToClient {
	fn from(value: Shared) -> Self {
		ToClient::Shared(value)
	}
}
