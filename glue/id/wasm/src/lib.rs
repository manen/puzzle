pub use id_common::*;

pub const fn __str_parse(s: &str) -> u32 {
	let mut bytes = s.as_bytes();
	let mut val = 0;
	while let [byte, rest @ ..] = bytes {
		assert!(b'0' <= *byte && *byte <= b'9', "invalid digit");
		val = val * 10 + (*byte - b'0') as u32;
		bytes = rest;
	}
	val
}

#[macro_export]
/// app_auto requires puzzle_common to be present
macro_rules! app_auto {
	() => {{
		::id::App {
			name: ::std::borrow::Cow::Borrowed(env!("CARGO_PKG_NAME")),
			version: (
				::id::__str_parse(env!("CARGO_PKG_VERSION_MAJOR")),
				::id::__str_parse(env!("CARGO_PKG_VERSION_MINOR")),
				::id::__str_parse(env!("CARGO_PKG_VERSION_PATCH")),
			),
			api_version: ::puzzle_common::API_VERSION,
		}
	}};
}

extern "C" {
	fn puzzle_id_api_version_major() -> u32;
	fn puzzle_id_api_version_minor() -> u32;
	fn puzzle_id_api_version_patch() -> u32;
}

pub static mut APP: Option<&'static App> = None;

pub fn runner() -> Runner {
	Runner {
		api_version: (
			unsafe { puzzle_id_api_version_major() },
			unsafe { puzzle_id_api_version_minor() },
			unsafe { puzzle_id_api_version_patch() },
		),
	}
}
pub fn ensure_api_version() -> Result<()> {
	match unsafe { APP } {
		None => Err(Error::Uninit),
		Some(app) => {
			let runner = runner().api_version;
			let app = app.api_version;
			api_check(runner, app)
		}
	}
}

fn uninit<T>(a: T) -> T {
	log::error!(
		"puzzle id api called while uninitialized:\ninitialize id using id::init(&'static App)"
	);
	a
}

#[no_mangle]
extern "C" fn puzzle_id_name_len() -> u32 {
	match unsafe { APP } {
		Some(app) => app.name.len() as u32,
		None => uninit(0),
	}
}

#[no_mangle]
extern "C" fn puzzle_id_name() -> *const u8 {
	match unsafe { APP } {
		Some(app) => app.name.as_ptr(),
		None => uninit(std::ptr::null()),
	}
}

#[no_mangle]
extern "C" fn puzzle_id_version_major() -> u32 {
	match unsafe { APP } {
		Some(app) => app.version.0,
		None => uninit(0),
	}
}
#[no_mangle]
extern "C" fn puzzle_id_version_minor() -> u32 {
	match unsafe { APP } {
		Some(app) => app.version.1,
		None => uninit(0),
	}
}
#[no_mangle]
extern "C" fn puzzle_id_version_patch() -> u32 {
	match unsafe { APP } {
		Some(app) => app.version.2,
		None => uninit(0),
	}
}

#[no_mangle]
extern "C" fn puzzle_id_target_api_version_major() -> u32 {
	match unsafe { APP } {
		Some(app) => app.api_version.0,
		None => uninit(0),
	}
}
#[no_mangle]
extern "C" fn puzzle_id_target_api_version_minor() -> u32 {
	match unsafe { APP } {
		Some(app) => app.api_version.1,
		None => uninit(0),
	}
}
#[no_mangle]
extern "C" fn puzzle_id_target_api_version_patch() -> u32 {
	match unsafe { APP } {
		Some(app) => app.api_version.2,
		None => uninit(0),
	}
}

pub fn init(app: &'static App) {
	unsafe { APP = Some(app) }
}
