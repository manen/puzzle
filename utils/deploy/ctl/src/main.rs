use std::process;

use clap::Parser;
use deploy_common::{Ctl, Resp};
use rust_socketio::{ClientBuilder, Payload, Socket};

fn run() -> anyhow::Result<()> {
	simple_logger::init_with_level(log::Level::Debug)?;

	let ctl = Ctl::parse();
	log::debug!("{ctl:?}");

	let callback = {
		let ctl = ctl.clone();
		move |payload: Payload, _: Socket| {
			log::debug!("incoming: {payload:?}");
			let message = match payload {
				Payload::String(s) => {
					log::error!("unexpected string in socketio payload: {s}");
					process::exit(1);
				}
				Payload::Binary(bin) => bincode::deserialize(&bin),
			};
			let message: Resp = match message {
				Ok(a) => a,
				Err(err) => {
					log::error!("failed to deserialize bincode payload: {err}");
					process::exit(1);
				}
			};
			match message {
				Ok(_) => log::info!("deployed {ctl:?}"),
				Err(err) => {
					log::error!("failed to deploy {ctl:?}: {err}");
					process::exit(1);
				}
			}
			process::exit(0);
		}
	};

	let socket = ClientBuilder::new(format!("http://{}", deploy_common::ADDR))
		.on("message", callback)
		.on("error", |err, _| {
			log::error!("{err:?}");
			process::exit(1);
		})
		.connect()?;

	std::thread::sleep(std::time::Duration::from_millis(100)); // how the fuck does my entire infrastructure rely on sleeping 100ms

	let encoded: Vec<u8> = bincode::serialize(&ctl)?;
	socket.emit("message", encoded)?;

	Ok(())
}

fn main() {
	std::thread::spawn(run);

	loop {
		std::thread::sleep(std::time::Duration::from_millis(100));
	}
}
