use std::{thread, time::Duration};

fn main() -> anyhow::Result<()> {
	simple_logger::init_with_level(log::Level::Info)?;
	let client = net_client::Client::new(format!("http://{}", net_common::ADDR))?;

	loop {
		client.ping()?;
		log::info!("pinged");
		thread::sleep(Duration::from_millis(500));
	}
}
