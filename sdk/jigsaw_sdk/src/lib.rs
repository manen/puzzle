use thiserror::Error;

extern "C" {
	fn jigsaw_start() -> i32;
	fn jigsaw_width() -> i32;
	fn jigsaw_height() -> i32;
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
pub fn width() -> Result<u32> {
	let w = unsafe { jigsaw_width() };
	if w < 0 {
		Err(Error::NotRunning)
	} else {
		Ok(w as u32)
	}
}
pub fn height() -> Result<u32> {
	let h = unsafe { jigsaw_height() };
	if h < 0 {
		Err(Error::NotRunning)
	} else {
		Ok(h as u32)
	}
}
pub fn size() -> Result<(u32, u32)> {
	let w = width()?;
	let h = height()?;
	Ok((w, h))
}

pub fn debug_text(x: u32, y: u32, msg: &str) -> Result<()> {
	if unsafe { jigsaw_debug_text(x, y, msg.as_ptr(), msg.len() as u32) } == 0 {
		Ok(())
	} else {
		Err(Error::NotRunning)
	}
}
