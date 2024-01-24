use rust_socketio::{ClientBuilder, Payload, Socket};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("socketio error: {0}")]
	SocketIO(#[from] rust_socketio::Error),
	#[error("bincode error: {0}")]
	Bincode(#[from] bincode::Error),
	#[error("serde_json error: {0}")]
	SerdeJson(#[from] serde_json::Error),
}
pub type Result<T> = std::result::Result<T, Error>;

pub struct Client {
	socket: rust_socketio::client::Client,
}
impl Client {
	pub fn default() -> Result<Self> {
		Client::new(puzzle_common::CONFIG.addr)
	}
	pub fn new<S: Into<String>>(addr: S) -> Result<Self> {
		let callback = |payload: Payload, socket: Socket| {
			log::debug!("incoming: {payload:?}");
			let message: Result<net_common::ToClient> = match payload {
				Payload::String(str) => serde_json::de::from_str(&str).map_err(|x| x.into()),
				Payload::Binary(bin_data) => bincode::deserialize(&bin_data).map_err(|x| x.into()),
			};
			let message = match message {
				Ok(a) => a,
				Err(err) => {
					log::warn!("failed to deserialize incoming packet: {err}");
					return;
				}
			};
			log::debug!("{message:?}");

			let resp: Option<net_common::ToClient> = match message {
				net_common::ToClient::Shared(shared) => shared.handle(),
			};

			// megyek cigizni

			if let Some(resp) = resp {
				let encoded: Vec<u8> = bincode::serialize(&resp).unwrap();
				match socket
					.emit("message", encoded)
					.map_err(|x| <rust_socketio::Error as Into<Error>>::into(x))
				{
					Ok(a) => a,
					Err(err) => {
						log::warn!("net_client: failed to serialize outgoing packet: {err}");
						return;
					}
				}
			}
		};

		let socket = ClientBuilder::new(addr)
			.on("message", callback)
			.on("error", |err, _| log::warn!("{err:?}"))
			.connect()?;

		Ok(Client { socket })
	}
	pub fn ping(&self) -> Result<()> {
		let packet = net_common::ToServer::Shared(net_common::Shared::Ping);
		let encoded: Vec<u8> = bincode::serialize(&packet)?;
		self.socket.emit("message", encoded)?;

		Ok(())
	}
}
