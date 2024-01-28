use anyhow::anyhow;
use axum::routing::get;
use once_cell::sync::Lazy;
use socketioxide::{
	extract::{Bin, SocketRef},
	SocketIo,
};
use std::{
	borrow::{Borrow, BorrowMut},
	cell::RefCell,
	process::{Child, Command},
	sync::{Arc, Mutex},
};

macro_rules! handle {
	($err:expr) => {
		match $err {
			Ok(_) => (),
			Err(err) => {
				log::error!("{err}");
				()
			}
		}
	};
}

#[derive(Debug)]
enum State {
	Stopped,
	Started(Child),
}
static STATE: Lazy<Mutex<State>> = Lazy::new(|| Mutex::new(State::Stopped));

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	real_main().await
}
async fn real_main() -> anyhow::Result<()> {
	simple_logger::init_with_level(log::Level::Debug).map_err(|err| anyhow!("{err}"))?;

	let (layer, io) = SocketIo::new_layer();
	io.ns("/", on_connect);

	let app = axum::Router::new()
		.route("/", get(|| async { "websocket magic" }))
		.layer(layer);

	let listener = tokio::net::TcpListener::bind(deploy_common::ADDR).await?;
	axum::serve(listener, app).await?;
	log::info!("server running on {}", deploy_common::ADDR);

	Ok(())
}

async fn on_connect(socket: SocketRef) {
	log::info!("new connection");
	socket.on("message", message);
}
async fn message(socket: SocketRef, bin: Bin) {
	handle!(real_message(socket, bin).await);
}
async fn real_message(socket: SocketRef, Bin(bin): Bin) -> anyhow::Result<()> {
	let messages = bin.into_iter().map(|bin| bincode::deserialize(&bin));

	for res in messages {
		let message: deploy_common::Ctl = res?;
		log::info!("{message:?}");

		let launch_new = || {
			let path = message.x.to_string_lossy();
			Command::new(path.as_ref()).spawn()
		};

		{
			let mut state_changer = STATE.lock().unwrap();
			let a = (*state_changer).borrow_mut();
			*a = match a {
				State::Stopped => State::Started(launch_new()?),
				State::Started(child) => {
					child.kill()?;
					State::Started(launch_new()?)
				}
			};
			log::info!("{a:?}");
		}
	}
	Ok(())
}
