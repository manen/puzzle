use anyhow::anyhow;
use fs_socketio_common::{RemoteSocketID, Request, Response};
use io::{Read, Write};
use socketioxide::extract::{Bin, SocketRef};
use std::{borrow::Cow, collections::HashMap, io, sync::Arc};
use tokio::sync::Mutex;

macro_rules! handle {
	(async $block:block) => {
		handle!((|| async $block)().await);
	};
	($block:block) => {
		handle!((|| $block)());
	};
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

#[derive(Clone, Debug)]
pub struct State<S: fs_trait::Socket> {
	socks: HashMap<RemoteSocketID, S>,
}
impl<S: fs_trait::Socket> Default for State<S> {
	fn default() -> Self {
		State {
			socks: std::iter::empty().collect(),
		}
	}
}
impl<S: fs_trait::Socket> State<S> {
	/// gen_id generates an id that is guaranteed to be free
	fn gen_id(&self) -> RemoteSocketID {
		let gen_id = || fastrand::u64(u64::MIN..u64::MAX);
		let mut id = gen_id();
		while self.socks.contains_key(&id) {
			id = gen_id();
		}
		id
	}
	fn open(&mut self, sock: S) -> fs_trait::Result<RemoteSocketID> {
		let id = self.gen_id();
		self.socks.insert(id, sock);

		Ok(id)
	}
	fn sock(&mut self, id: RemoteSocketID) -> Option<&mut S> {
		self.socks.get_mut(&id)
	}
	fn take_sock(&mut self, id: RemoteSocketID) -> Option<S> {
		self.socks.remove(&id)
	}
}

pub trait BindFs {
	fn bind_fs<Fs: fs_trait::Fs + Send + Sync>(&self, ns: impl Into<Cow<'static, str>>, fs: Fs)
	where
		Fs: 'static,
		Fs::Socket: Send + Sync + 'static;
}
impl BindFs for socketioxide::SocketIo {
	fn bind_fs<Fs: fs_trait::Fs + Send + Sync>(&self, ns: impl Into<Cow<'static, str>>, fs: Fs)
	where
		Fs: 'static,
		Fs::Socket: Send + Sync + 'static,
	{
		let state: Arc<Mutex<State<Fs::Socket>>> = Arc::new(Mutex::new(Default::default()));
		let fs: Arc<Mutex<Fs>> = Arc::new(Mutex::new(fs));

		let on_message = |socket: SocketRef, Bin(bin)| async move {
			handle!(async {
				let reqs = bin
					.into_iter()
					.map(|bin| bincode::deserialize(&bin))
					.collect::<Result<Vec<Request>, _>>()?;

				for req in reqs {
					let resp = match req {
						Request::ReadDir { path } => {
							let fs = fs.lock().await;
							Some(Response::ReadDir {
								dir: fs.read_dir(&path).await.map(|dir| dir.collect()),
							})
						}
						Request::Open { path } => {
							let native_sock = {
								let fs = fs.lock().await;
								fs.open(&path).await
							};
							let sock = {
								let mut state = state.lock().await;
								match native_sock {
									Ok(a) => state.open(a),
									Err(err) => Err(err),
								}
							};
							Some(Response::Open { sock })
						}
						Request::Read { sock, len } => {
							let buf = {
								let mut state = state.lock().await;

								match state.sock(sock) {
									None => Err(fs_socketio_common::Error::InvalidSock {
										sock,
										op: fs_socketio_common::Operation::Read,
									}),
									Some(sock) => {
										let mut buf = vec![0; len as usize];
										match sock.read_exact(&mut buf) {
											Ok(_) => Ok(buf),
											Err(err) => {
												Err(fs_socketio_common::Error::IoErr(err.into()))
											}
										}
									}
								}
							};

							Some(Response::Read {
								sock,
								buf: buf.map(|slice| slice.into()),
							})
						}
						Request::Write { sock, buf } => {
							let len = {
								let mut state = state.lock().await;

								match state.sock(sock) {
									None => Err(fs_socketio_common::Error::InvalidSock {
										sock,
										op: fs_socketio_common::Operation::Write,
									}),
									Some(sock) => match sock.write(&buf) {
										Ok(len) => Ok(len as u32),
										Err(err) => {
											Err(fs_socketio_common::Error::IoErr(err.into()))
										}
									},
								}
							};

							Some(Response::Write { sock, len })
						}
						Request::Close { sock } => {
							{
								let mut state = state.lock().await;
								state.take_sock(sock); // we don't need to check the returned option cause it gets dropped either way
							}
							None
						}
					};
					if let Some(resp) = resp {
						let encoded = bincode::serialize(&resp)?;
						socket
							.bin(vec![encoded])
							.emit("message", [] as [u8; 0])
							.map_err(|err| anyhow!("{err}"))?;
					}
				}

				Ok::<(), anyhow::Error>(())
			});
			todo!()
		};
		let on_connect = |socket: SocketRef| {
			socket.on("message", on_message);
		};

		self.ns(ns, on_connect);
	}
}
