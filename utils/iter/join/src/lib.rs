use std::iter::Peekable;

enum Next {
	A,
	B,
}
impl Next {
	fn flip(&mut self) {
		*self = match self {
			Next::A => Next::B,
			Next::B => Next::A,
		}
	}
}
pub struct Join<T, A: Iterator<Item = T>, B: Iterator<Item = T>> {
	a: Peekable<A>,
	b: B,
	next: Next,
}
impl<T, A: Iterator<Item = T>, B: Iterator<Item = T>> Iterator for Join<T, A, B> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		let next = match self.next {
			Next::A => self.a.next(),
			Next::B => {
				if self.a.peek().is_some() {
					self.b.next()
				} else {
					None
				}
			}
		};
		self.next.flip();
		next
	}
	fn size_hint(&self) -> (usize, Option<usize>) {
		let a = self.a.size_hint();
		let b = self.b.size_hint();

		(
			a.0 + b.0,
			match (a.1, b.1) {
				(None, None) => None,
				(Some(a), None) => Some(a + b.0),
				(None, Some(b)) => Some(b + a.0),
				(Some(a), Some(b)) => Some(a + b),
			},
		)
	}
}

pub trait JoinIter<T>: Iterator<Item = T> + Sized {
	fn join_iter<B: IntoIterator<Item = T>>(self, b: B) -> Join<T, Self, B::IntoIter>;
}
impl<T, A: Iterator<Item = T>> JoinIter<T> for A {
	fn join_iter<B: IntoIterator<Item = T>>(self, b: B) -> Join<T, Self, B::IntoIter> {
		Join {
			a: self.peekable(),
			b: b.into_iter(),
			next: Next::A,
		}
	}
}

pub trait JoinItem<T: Clone>: Iterator<Item = T> + Sized {
	fn join(self, item: T) -> Join<T, Self, std::iter::Repeat<T>>;
}
impl<T: Clone, A: Iterator<Item = T>> JoinItem<T> for A {
	fn join(self, item: T) -> Join<T, Self, std::iter::Repeat<T>> {
		self.join_iter(std::iter::repeat(item))
	}
}

#[test]
fn join_iter() {
	let words: [&str; 3] = ["hello", "name", "world"];
	let others: [&str; 2] = ["my", "is"];

	let total = words.into_iter().join_iter(others).collect::<Vec<&str>>();

	assert_eq!(total, &["hello", "my", "name", "is", "world"]);
}

#[test]
fn join_item() {
	let words: [&str; 3] = ["hello", "i'm", "testing"];
	let text = words.into_iter().join(" ").collect::<String>();

	assert_eq!(text, "hello i'm testing");
}

#[test]
fn join_all() {
	let words_1: [&str; 3] = ["hello", "name", "world"];
	let words_2: [&str; 2] = ["my", "is"];

	let words = words_1.into_iter().join_iter(words_2);
	let text = words.join(" ").collect::<String>();

	assert_eq!(text, "hello my name is world");
}

#[test]
fn join_nums() {
	let nums_1: [i32; 3] = [0, 2, 4];
	let nums_2: [i32; 3] = [1, 3, 5];
	let nums = nums_1.into_iter().join_iter(nums_2).collect::<Vec<i32>>();

	assert_eq!(nums, &[0, 1, 2, 3, 4]);
}
