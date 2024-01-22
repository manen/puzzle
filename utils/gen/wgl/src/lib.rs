use crate::filter::IntoFilter;
use iter_join::JoinItem;
use std::fs;

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

pub fn api() {
	let relevant_c = fs::read_to_string("/usr/include/GLES3/gl3.h")
		.unwrap()
		.split('\n')
		.filter_relevant()
		.join("\n")
		.collect::<String>();
	let relevant_c = relevant_c.as_str();
	let tokenizer = token::tokenizer(&relevant_c);
	panic!("{tokenizer:?}");

	// tokenizer.for_each(|a| eprintln!("{a:?}")); // this still hangs forever
	// todo!()
}
