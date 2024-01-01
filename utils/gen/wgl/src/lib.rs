mod api;
mod filter;
mod token;

// pub struct Api<'a, I: Iterator<Item = &'a str>> {
// 	iter: I,
// }
// impl<'a, I: Iterator<Item = &'a str>> Iterator for Api<'a, I> {
// 	type Item = api::Def;

// 	fn next(&mut self) -> Option<Self::Item> {}
// }

pub fn api() {}

#[test]
fn fasz() {
	use std::fs;

	use filter::IntoFilter;
	use iter_join::JoinItem;

	let a = fs::read_to_string("/usr/include/GLES3/gl3.h")
		.unwrap()
		.split('\n')
		.filter_relevant()
		.join("\n")
		.collect::<String>();

	panic!("{a}");
}
