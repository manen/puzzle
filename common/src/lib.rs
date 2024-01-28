use std::borrow::Cow;

pub const API_VERSION: (u32, u32, u32) = (0, 0, 0);

pub struct Config<'a> {
	pub addr: Cow<'a, str>,
}
