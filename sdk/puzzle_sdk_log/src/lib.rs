use std::fmt;

use log::{Level, LevelFilter, SetLoggerError};

extern "C" {
	fn puzzle_log(level: u32, ptr: *const u8, len: u32);
	fn puzzle_log_flush();
}

struct PuzzleLogger;
impl log::Log for PuzzleLogger {
	fn enabled(&self, _: &log::Metadata) -> bool {
		true
	}

	fn log(&self, record: &log::Record) {
		if self.enabled(record.metadata()) {
			let s = fmt::format(*record.args());
			unsafe { puzzle_log(record.level() as u32, s.as_ptr(), s.len() as u32) };
		}
	}
	fn flush(&self) {
		unsafe { puzzle_log_flush() };
	}
}

static LOGGER: PuzzleLogger = PuzzleLogger;

pub fn init() -> Result<(), SetLoggerError> {
	log::set_logger(&LOGGER)?;
	log::set_max_level(LevelFilter::Trace);
	Ok(())
}
