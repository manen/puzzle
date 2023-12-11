#[no_mangle]
pub extern "C" fn puzzle_main() {
	puzzle_sdk_log::init().unwrap();

	log::info!("hello world");
}
