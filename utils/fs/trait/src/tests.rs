use crate::Fs;

#[test]
fn empty() {
	assert!(crate::empty().read_dir("leszarni").is_err());
}
