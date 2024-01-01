use super::*;

#[test]
fn empty_slice() {
	let slice: &[i32] = &[];
	let mut readu = slice.read_until();
	let read = readu.read_until(|a| *a == 3);

	assert_eq!(read, Read::Finished)
}

#[test]
fn nums() {
	let mut readu = [0, 1, 2, 2, 1, 0, 1, 2, 0].read_until();

	assert_eq!(readu.read_until(|a| *a == 2), Read::Condition(&[0, 1]));
	assert_eq!(readu.read_until(|a| *a == 0), Read::Condition(&[2, 1]));
	assert_eq!(readu.read_until(|a| *a == 0), Read::Condition(&[1, 2]));
	assert_eq!(readu.read_until(|_| false), Read::Finished);
}
