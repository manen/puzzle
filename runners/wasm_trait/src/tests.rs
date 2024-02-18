use super::*;

fn check<A: Args>(_: A) {}

#[test]
fn test() {
	check(4);
	check((3 as i32, 4));
	check((5, 6, 7));
	check(3.1);
	check((3.1, 4, 5.6))
}

#[test]
fn test_argdyn() {
	assert_eq!(
		(3, 4, 5, 6, 7).args().collect::<Vec<_>>(),
		[3, 4, 5, 6, 7]
			.into_iter()
			.map(|num| ArgDyn::I32(num))
			.collect::<Vec<_>>(),
	);
}
