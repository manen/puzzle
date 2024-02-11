use std::{error::Error, fmt::Display, io};

use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, IoErr>;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IoErr(String);
impl From<io::Error> for IoErr {
	fn from(value: io::Error) -> Self {
		IoErr(format!("{value}"))
	}
}
impl Display for IoErr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}
impl Error for IoErr {
	fn description(&self) -> &str {
		&self.0
	}
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		None
	}
	fn cause(&self) -> Option<&dyn Error> {
		None
	}
}
