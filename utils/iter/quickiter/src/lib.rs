pub struct Quickiter<T, F: FnMut() -> Option<T>> {
	f: F,
}
impl<T, F: FnMut() -> Option<T>> Iterator for Quickiter<T, F> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		(self.f)()
	}
}

pub fn iter<T, F: FnMut() -> Option<T>>(f: F) -> Quickiter<T, F> {
	Quickiter { f }
}
pub fn iter_infinite<T, F: FnMut() -> T>(mut f: F) -> Quickiter<T, impl FnMut() -> Option<T>> {
	Quickiter {
		f: move || Some(f()),
	}
}
