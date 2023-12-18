use crate::{Error, Result, RuntimeInit};
use bgfx_rs::bgfx;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

#[derive(Default)]
pub enum Runtime {
	#[default]
	Uninit,
	Init(RuntimeInit),
}
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
			Runtime::Init(_) => Err(Error::AlreadyInit),
		}
	}
	pub fn debug_text(&mut self, x: u32, y: u32, text: &str) -> Result<()> {
		match self {
			Runtime::Uninit => Err(Error::Uninit),
			Runtime::Init(rt) => Ok(rt.debug_text(x, y, text)),
		}
	}

	pub fn runtime(&mut self) -> Result<&mut RuntimeInit> {
		match self {
			Runtime::Uninit => Err(Error::Uninit),
			Runtime::Init(rt) => Ok(rt),
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
