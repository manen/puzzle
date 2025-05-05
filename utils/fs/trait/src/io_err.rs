use std::{error::Error, fmt::Display, io};

pub type Result<T> = std::result::Result<T, IoErr>;
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
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
