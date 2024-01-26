use std::io;

use crate::prelude::*;

#[test]
fn empty() {
	assert!(crate::empty().read_dir("leszarni").is_err());
}

#[test]
fn file_mount() {
	let fs = crate::empty().mount_file("csoki.txt", "csocs".as_bytes().read_only());

	assert!(fs.open("csocs").is_err());
	assert_eq!(
		io::read_to_string(fs.open("csoki.txt").unwrap()).unwrap(),
		"csocs"
	);
}
