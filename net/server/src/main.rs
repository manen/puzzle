use anyhow::anyhow;
use fs_socketio_server::BindFs;
use fs_trait::prelude::*;
use socketioxide::{
	extract::{Bin, SocketRef},
	SocketIo,
};
use tower_http::services::ServeDir;

macro_rules! handle {
	($err:expr) => {
		match $err {
			Ok(_) => (),
			Err(err) => {
				log::warn!("{err}");
				()
			}
		}
	};
}

async fn on_connect(socket: SocketRef) {
	handle!((|| {
		log::debug!("socket connected: {}", socket.id);

		socket.on("message", |socket: SocketRef, Bin(bin)| {
			handle!((|| -> anyhow::Result<()> {
				let messages = bin
					.into_iter()
					.map(|bin| bincode::deserialize(&bin).map_err(|err| anyhow!("{err}")));

				for res in messages {
					let message: net_common::ToServer = res?;
					log::debug!("{message:?}");
					let resp: Option<net_common::ToServer> = match message {
						net_common::ToServer::Shared(shared) => shared.handle(),
					};
					log::debug!("resp: {resp:?}");

					if let Some(resp) = resp {
						let encoded = bincode::serialize(&resp)?;
						match socket.bin(vec![encoded]).emit("message", [] as [u8; 0]) {
							Ok(a) => a,
							Err(err) => return Err(anyhow!("{err}")),
						}
					}
				}
				Ok(())
			})());
		});

		let encoded = bincode::serialize(&net_common::ToClient::Shared(net_common::Shared::Ping))?;
		handle!(socket.bin(vec![encoded]).emit("message", [] as [u8; 0]));

		Ok::<(), anyhow::Error>(())
	})())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	simple_logger::init_with_level(log::Level::Debug)?;

	let (layer, io) = SocketIo::new_layer();
	io.ns("/", on_connect);
	io.bind_fs(
		"/fs",
		fs_trait::empty().mount_file("README.md", "# did i do it?".as_bytes().read_only()),
	);

	let app = axum::Router::new()
		.nest_service("/", ServeDir::new("static"))
		.layer(layer);

	// axum::Server::bind(&"0.0.0.0:4200".parse().unwrap())
	// 	.serve(app.into_make_service())
	// 	.await?;

	let listener = tokio::net::TcpListener::bind(net_common::ADDR).await?;
	axum::serve(listener, app).await?;
	log::info!("server running on {}", net_common::ADDR);

	Ok(())
}
