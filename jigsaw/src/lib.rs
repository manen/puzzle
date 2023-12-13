use bgfx_rs::bgfx::{self, DbgTextClearArgs, ResetArgs};
use thiserror::Error;

mod init;
pub use init::*;

#[cfg(feature = "wasm")]
pub mod wasm;

#[derive(Debug, Error)]
pub enum Error {
	#[error("already initialized")]
	AlreadyInit,
	#[error("glfw init error: {0}")]
	GlfwInit(#[from] glfw::InitError),
	#[error("failed to create glfw window")]
	GlfwCreateWindow,
	#[error("failed to init bgfx")]
	BgfxInit,
	#[error("jigsaw is not initialized")]
	Uninit,

	#[cfg(feature = "wasm")]
	#[error("wasmtime error: {0}")]
	Wasmtime(#[from] wasmtime::Error),
}
pub type Result<T> = std::result::Result<T, Error>;

pub struct RuntimeInit {
	old_size: (i32, i32),
	glfw: glfw::Glfw,
	window: glfw::PWindow,
	events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
}
impl RuntimeInit {
	pub fn debug_text(&self, x: u32, y: u32, text: &str) {
		bgfx::dbg_text(x as _, y as _, 0x0f, text)
	}

	pub fn should_close(&self) -> bool {
		self.window.should_close()
	}
	pub fn frame(&mut self) {
		self.glfw.poll_events();
		for (_, event) in glfw::flush_messages(&self.events) {
			if let glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) = event {
				self.window.set_should_close(true)
			}
		}

		let size = self.window.get_framebuffer_size();
		if self.old_size != size {
			bgfx::reset(size.0 as _, size.1 as _, ResetArgs::default())
		}

		bgfx::set_view_rect(0, 0, 0, size.0 as _, size.1 as _);
		bgfx::touch(0);

		bgfx::dbg_text_clear(DbgTextClearArgs::default());
	}
	pub fn frame_post(&mut self) {
		bgfx::frame(false);
	}
}
