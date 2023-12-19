#[no_mangle]
pub extern "C" fn puzzle_main() {
	puzzle_sdk_log::init().unwrap();
	log::info!("hello world");

	match main() {
		Ok(_) => (),
		Err(err) => log::error!("{err}"),
	}
}

fn main() -> anyhow::Result<()> {
	Ok(())
}

#[no_mangle]
pub extern "C" fn puzzle_render() {
	match render() {
		Ok(_) => (),
		Err(err) => log::error!("{err}"),
	}
}

fn render() -> anyhow::Result<()> {
	log::info!("haiiii:3");

	Ok(())
}
