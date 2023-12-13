use bgfx_rs::bgfx::{self, DbgTextClearArgs, ResetArgs};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use thiserror::Error;

#[cfg(feature = "wasm")]
pub mod wasm;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

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

#[derive(Default)]
pub enum Runtime {
	#[default]
	Uninit,
	Init(RuntimeInit),
}
struct RuntimeInit {
	old_size: (i32, i32),
	glfw: glfw::Glfw,
	window: glfw::PWindow,
	events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
}
impl RuntimeInit {}
impl Runtime {
	pub fn start(&mut self) -> Result<()> {
		match self {
			Runtime::Uninit => {
				let mut glfw = glfw::init(glfw::fail_on_errors)?;
				glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

				let (mut window, events) = glfw
					.create_window(WIDTH, HEIGHT, "jigsaw", glfw::WindowMode::Windowed)
					.ok_or(Error::GlfwCreateWindow)?;
				window.set_key_polling(true);

				let mut init = bgfx::Init::new();

				init.type_r = get_render_type();
				init.resolution.width = WIDTH as u32;
				init.resolution.height = HEIGHT as u32;
				init.resolution.reset = bgfx::ResetFlags::VSYNC.bits();
				init.platform_data = get_platform_data(&window);

				if !bgfx::init(&init) {
					Err(Error::BgfxInit)?
				};

				bgfx::set_debug(bgfx::DebugFlags::TEXT.bits());
				bgfx::set_view_clear(
					0,
					bgfx::ClearFlags::COLOR.bits() | bgfx::ClearFlags::DEPTH.bits(),
					bgfx::SetViewClearArgs {
						rgba: 0x103030ff,
						..Default::default()
					},
				);

				let old_size = window.get_framebuffer_size();

				*self = Runtime::Init(RuntimeInit {
					old_size,
					glfw,
					window,
					events,
				});
				Ok(())
			}
			Runtime::Init { .. } => Err(Error::AlreadyInit),
		}
	}
	pub fn debug_text(&mut self, x: u32, y: u32, msg: &str) -> i32 {
		match self.debug_text_impl(x, y, msg) {
			Ok(_) => 0,
			Err(err) => {
				log::warn!("{err}");
				-1
			}
		}
	}
	fn debug_text_impl(&mut self, x: u32, y: u32, text: &str) -> Result<()> {
		match &self {
			Runtime::Uninit => Err(Error::Uninit),
			Runtime::Init { .. } => {
				bgfx::dbg_text(x as u16, y as u16, 0x0f, text);
				Ok(())
			}
		}
	}

	pub fn should_close(&self) -> Result<bool> {
		match &self {
			Runtime::Uninit => Err(Error::Uninit),
			Runtime::Init(RuntimeInit { window, .. }) => Ok(window.should_close()),
		}
	}
	pub fn frame(&mut self) -> Result<()> {
		match self {
			Runtime::Uninit => Err(Error::Uninit),
			Runtime::Init(RuntimeInit {
				old_size,
				glfw,
				window,
				events,
			}) => {
				glfw.poll_events();
				for (_, event) in glfw::flush_messages(&events) {
					if let glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) =
						event
					{
						window.set_should_close(true)
					}
				}

				let size = window.get_framebuffer_size();
				if old_size != &size {
					bgfx::reset(size.0 as _, size.1 as _, ResetArgs::default())
				}

				bgfx::set_view_rect(0, 0, 0, size.0 as _, size.1 as _);
				bgfx::touch(0);

				bgfx::dbg_text_clear(DbgTextClearArgs::default());

				Ok(())
			}
		}
	}
	pub fn frame_post(&mut self) -> Result<()> {
		match self {
			Runtime::Uninit => Err(Error::Uninit),
			Runtime::Init { .. } => {
				bgfx::frame(false);
				Ok(())
			}
		}
	}

	pub fn uninit(&mut self) -> Result<()> {
		match &self {
			Runtime::Uninit => Err(Error::Uninit),
			Runtime::Init { .. } => {
				bgfx::shutdown();
				*self = Runtime::Uninit;
				Ok(())
			}
		}
	}
}

fn get_platform_data(window: &glfw::Window) -> bgfx::PlatformData {
	let mut pd = bgfx::PlatformData::new();

	match window.raw_window_handle() {
		#[cfg(any(
			target_os = "linux",
			target_os = "dragonfly",
			target_os = "freebsd",
			target_os = "netbsd",
			target_os = "openbsd"
		))]
		RawWindowHandle::Xlib(data) => {
			pd.nwh = data.window as *mut _;
		}
		#[cfg(any(
			target_os = "linux",
			target_os = "dragonfly",
			target_os = "freebsd",
			target_os = "netbsd",
			target_os = "openbsd"
		))]
		RawWindowHandle::Wayland(data) => {
			pd.ndt = data.surface; // same as window, on wayland there ins't a concept of windows
		}

		#[cfg(target_os = "macos")]
		RawWindowHandle::AppKit(data) => {
			pd.nwh = data.ns_window;
		}
		#[cfg(target_os = "windows")]
		RawWindowHandle::Win32(data) => {
			pd.nwh = data.hwnd;
		}
		_ => panic!("Unsupported Window Manager"),
	}

	return pd;
}

#[cfg(target_os = "linux")]
fn get_render_type() -> bgfx::RendererType {
	bgfx::RendererType::OpenGL
}

#[cfg(not(target_os = "linux"))]
fn get_render_type() -> bgfx::RendererType {
	bgfx::RendererType::Count
}
