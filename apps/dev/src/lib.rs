use id::App;

const APP: App = id::app_auto!();

#[no_mangle]
pub extern "C" fn puzzle_main() {
	puzzle_log::init().unwrap();
	match main() {
		Ok(_) => (),
		Err(err) => log::error!("{err}"),
	}
}

fn main() -> anyhow::Result<()> {
	id::init(&APP);
	id::ensure_api_version()?;

	log::info!("hello world");
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
