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

	assert_eq!(readu.read_until(|a| a == b' '), Read::Condition("hello"));
	assert_eq!(readu.read_until(|a| a == b' '), Read::End("world!"))
}
