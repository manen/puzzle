use thiserror::Error;

extern "C" {
	fn jigsaw_start() -> i32;
	fn jigsaw_debug_text(x: u32, y: u32, ptr: *const u8, len: u32) -> i32;
}

#[derive(Error, Debug)]
pub enum Error {
	#[error("failed to start jigsaw")]
	Start,
	#[error("jigsaw hasn't been started")]
	NotRunning,
}
pub type Result<T> = std::result::Result<T, Error>;

pub fn start() -> Result<()> {
	if unsafe { jigsaw_start() } == 0 {
		Ok(())
	} else {
		Err(Error::Start)
	}
}

pub fn debug_text(x: u32, y: u32, msg: &str) -> Result<()> {
	if unsafe { jigsaw_debug_text(x, y, msg.as_ptr(), msg.len() as u32) } == 0 {
		Ok(())
	} else {
		Err(Error::NotRunning)
	}
}
