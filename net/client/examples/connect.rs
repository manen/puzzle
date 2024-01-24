use std::{thread, time::Duration};

fn main() -> anyhow::Result<()> {
	simple_logger::init_with_level(log::Level::Info)?;
	let client = net_client::Client::default()?;

	loop {
		client.ping()?;
		thread::sleep(Duration::from_millis(500));
	}
}
