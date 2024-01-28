use std::io;

pub(crate) mod prelude {
	pub use super::{IntoReadOnly, IntoWriteOnly};
}

pub trait IntoReadOnly: Sized + io::Read {
	fn read_only(self) -> ReadOnly<Self> {
		ReadOnly { r: self }
	}
}
impl<R: io::Read> IntoReadOnly for R {}
pub trait IntoWriteOnly: Sized + io::Write {
	fn write_only(self) -> WriteOnly<Self> {
		WriteOnly { w: self }
	}
}
impl<W: io::Write> IntoWriteOnly for W {}

#[derive(Copy, Clone, Debug)]
pub struct ReadOnly<R: io::Read> {
	r: R,
}
impl<R: io::Read> io::Read for ReadOnly<R> {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		self.r.read(buf)
	}
}
impl<R: io::Read> io::Write for ReadOnly<R> {
	fn write(&mut self, _: &[u8]) -> io::Result<usize> {
		Err(io::Error::new(
			io::ErrorKind::Unsupported,
			"readonly shouldn't be written to",
		))
	}
	fn flush(&mut self) -> io::Result<()> {
		Err(io::Error::new(
			io::ErrorKind::Unsupported,
			"readonly shouldn't be written to",
		))
	}
}

#[derive(Copy, Clone, Debug)]
pub struct WriteOnly<W: io::Write> {
	w: W,
}
impl<W: io::Write> io::Read for WriteOnly<W> {
	fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
		Err(io::Error::new(
			io::ErrorKind::Unsupported,
			"writeonly shouldn't be read from",
		))
	}
}
impl<W: io::Write> io::Write for WriteOnly<W> {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		self.w.write(buf)
	}
	fn flush(&mut self) -> io::Result<()> {
		self.w.flush()
	}
}
