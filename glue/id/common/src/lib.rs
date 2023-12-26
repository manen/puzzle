use std::{borrow::Cow, cmp::Ordering, fmt::Display};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct App {
	pub name: Cow<'static, str>,
	pub version: (u32, u32, u32),
	pub api_version: (u32, u32, u32),
}
impl Display for App {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{} {}.{}.{}",
			self.name, self.version.0, self.version.1, self.version.2
		)
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runner {
	pub api_version: (u32, u32, u32),
}

#[derive(Error, Debug)]
pub enum Error {
	#[error("outdated app api: {}.{}.{} (runner: {}.{}.{})", app.0, app.1, app.2, runner.0, runner.1, runner.2)]
	OutdatedApp {
		app: (u32, u32, u32),
		runner: (u32, u32, u32),
	},
	#[error("outdated runner api: {}.{}.{} (app: {}.{}.{})", runner.0, runner.1, runner.2, app.0, app.1, app.2)]
	OutdatedRunner {
		app: (u32, u32, u32),
		runner: (u32, u32, u32),
	},
	#[error("id is uninitalized: initialize it using id::init(&'static App)")]
	Uninit,
}
pub type Result<T> = std::result::Result<T, Error>;

pub fn api_check(runner: (u32, u32, u32), app: (u32, u32, u32)) -> Result<()> {
	let cmp = (
		app.0.cmp(&runner.0),
		app.1.cmp(&runner.1),
		app.2.cmp(&runner.2),
	);

	match cmp {
		(Ordering::Equal, Ordering::Equal, Ordering::Equal) => {
			log::debug!("app and runner api versions are equal");
			Ok(())
		}
		(Ordering::Equal, _, _) => {
			log::info!("app and runner api versions differ, but the major version is the same");
			Ok(())
		}
		(Ordering::Greater, _, _) => Err(Error::OutdatedRunner { app, runner }),
		(Ordering::Less, _, _) => Err(Error::OutdatedApp { app, runner }),
	}
}
