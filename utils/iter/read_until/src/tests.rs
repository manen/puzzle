use super::*;

#[test]
fn empty_slice() {
	let slice: &[i32] = &[];
	let mut readu = slice.reader();
	let read = readu.read_until(|a| *a == 3);

	assert_eq!(read, Read::Finished)
}

#[test]
fn nums() {
	let slice: &[i32] = &[0, 1, 2, 2, 1, 0, 1, 2, 0];
	let mut readu = slice.reader();

	assert_eq!(
		readu.read_until(|a| *a == 2),
		Read::Condition((&[0, 1]) as &[i32])
	);
	assert_eq!(
		readu.read_until(|a| *a == 0),
		Read::Condition((&[2, 1]) as &[i32])
	);
	assert_eq!(
		readu.read_until(|a| *a == 0),
		Read::Condition((&[1, 2]) as &[i32])
	);
	assert_eq!(readu.read_until(|_| false), Read::Finished);
}

#[test]
fn str() {
	let string = "hello world!";
	let mut readu = string.reader();

	assert_eq!(readu.read_until(|a| a.eq(&b' ')), Read::Condition("hello"));
	assert_eq!(readu.read_until(|a| a.eq(&b' ')), Read::End("world!"))
}

#[test]
fn str_item() {
	let string = "long words words blah blah brainrot";
	let mut readu = string.reader();
	let mut iter = quickiter::iter_infinite(move || readu.read_until_item(b' '));

	assert_eq!(iter.next(), Some(Read::Condition("long")));
	assert_eq!(iter.next(), Some(Read::Condition("words")));
	assert_eq!(iter.next(), Some(Read::Condition("words")));
	assert_eq!(iter.next(), Some(Read::Condition("blah")));
	assert_eq!(iter.next(), Some(Read::Condition("blah")));
	assert_eq!(iter.next(), Some(Read::End("brainrot")));
}

#[test]
fn iterr() {
	let mut readu = (0..10).chain(0..10).chain(0..10).reader();

	assert_eq!(readu.read_until_item(3), Read::Condition(vec![0, 1, 2]));
	assert_eq!(
		readu.read_until_item(9),
		Read::Condition(vec![4, 5, 6, 7, 8])
	);
	assert_eq!(
		readu.read_until_item(8),
		Read::Condition(vec![0, 1, 2, 3, 4, 5, 6, 7])
	);
	assert_eq!(
		readu.read_until(|_| false),
		Read::End(vec![9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
	);
}
