use std::io;

use crate::prelude::*;

#[tokio::main]
#[test]
async fn empty() {
	assert!(crate::empty().read_dir("leszarni").await.is_err());
}

#[tokio::main]
#[test]
async fn file_mount() {
	let fs = crate::empty()
		.mount_file("/csoki.txt", "csocs".as_bytes().read_only())
		.abs();

	assert!(fs.open("csocs").await.is_err());
	assert_eq!(
		io::read_to_string(fs.open("csoki.txt").await.unwrap()).unwrap(),
		"csocs"
	);
}

#[tokio::main]
#[test]
async fn fs_mount() {
	let inner = crate::empty()
		.mount_file(
			"/belso_geci.txt",
			"ez egy mappaval bentebb van! wow!".as_bytes().read_only(),
		)
		.abs();
	let a = crate::empty()
		.mount_file("/csocs.txt", "lopocs".as_bytes().read_only())
		.mount_file("/fasz.txt", "geci geci geci".as_bytes().read_only())
		.mount_fs("/inner", inner)
		.abs();

	assert_eq!(
		a.read_dir("/").await.unwrap().collect::<Vec<_>>(),
		vec!["/csocs.txt", "/fasz.txt", "/inner"]
	);
}

#[tokio::main]
#[test]
async fn remove_tail() {
	assert_eq!(crate::abs::remove_tail("csocs"), "csocs");
	assert_eq!(crate::abs::remove_tail("csocsok/"), "csocsok");
	assert_eq!(
		crate::abs::remove_tail("csocsok/es/segg".to_owned()),
		"csocsok/es/segg"
	);
	assert_eq!(
		crate::abs::remove_tail("csocsok/es/segg/".to_owned()),
		"csocsok/es/segg"
	);
}

#[tokio::main]
#[test]
async fn add_tail() {
	assert_eq!(crate::abs::add_tail("csocs"), "csocs/");
	assert_eq!(crate::abs::add_tail("csocsok/"), "csocsok/");
	assert_eq!(
		crate::abs::add_tail("csocsok/es/segg".to_owned()),
		"csocsok/es/segg/"
	);
	assert_eq!(
		crate::abs::add_tail("csocsok/es/segg/".to_owned()),
		"csocsok/es/segg/"
	);
}

#[tokio::main]
#[test]
async fn ridiculously_complicated() {
	let fs = crate::empty()
		.mount_file("/atoms", b"vibrating and shit".read_only())
		.mount_fs(
			"/earth",
			crate::empty()
				.mount_file("/weather", b"sunny".read_only())
				.mount_file("/snowing", b"false".read_only())
				.mount_fs(
					"/peter_griffin",
					crate::empty()
						.mount_file("/name", b"peter".read_only())
						.mount_file("/age", b"19".read_only()),
				),
		);

	assert_eq!(
		fs.read_dir("/").await.unwrap().collect::<Vec<_>>(),
		vec!["/atoms", "/earth"]
	);

	// TODO currently only reads /earth/ and not /earth
	// TODO i should like actually fix this but who am i kidding

	assert_eq!(
		fs.read_dir("/earth")
			.await
			.map_err(|err| format!("{err}"))
			.unwrap()
			.collect::<Vec<_>>(),
		vec!["/earth/weather", "/earth/snowing", "/earth/peter_griffin"]
	);
	assert_eq!(
		fs.read_dir("/earth/peter_griffin/")
			.await
			.unwrap()
			.collect::<Vec<_>>(),
		vec!["/earth/peter_griffin/name", "/earth/peter_griffin/age"]
	);
}
