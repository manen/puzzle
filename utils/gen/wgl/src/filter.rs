pub trait IntoFilter<'a>: Iterator<Item = &'a str> + Sized {
	fn filter_relevant(self) -> Filter<'a, Self>;
}
impl<'a, I: Iterator<Item = &'a str>> IntoFilter<'a> for I {
	fn filter_relevant(self) -> Filter<'a, Self> {
		Filter { iter: self }
	}
}
#[derive(Debug, Clone)]
pub struct Filter<'a, I: Iterator<Item = &'a str>> {
	iter: I,
}
impl<'a, I: Iterator<Item = &'a str>> Iterator for Filter<'a, I> {
	type Item = &'a str;

	fn next(&mut self) -> Option<Self::Item> {
		let next = self.iter.next()?;
		if next.starts_with("typedef")
			|| next.starts_with("#define")
			|| next.starts_with("GL_APICALL")
		{
			Some(next)
		} else {
			self.next()
		}
	}
}
