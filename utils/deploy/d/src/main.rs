use anyhow::anyhow;
use axum::routing::get;
use clap::Parser;
use deploy_common::Ctl;
use once_cell::sync::Lazy;
use socketioxide::{
	extract::{Bin, SocketRef},
	SocketIo,
};
use std::{
	borrow::BorrowMut,
	fs,
	path::PathBuf,
	process::{Child, Command},
	sync::Mutex,
};

mod systemd;
use systemd::systemd;

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
	/// stopped represents a state where no child process is running. the .deploy directory's contents are not guaranteed to be anything
	Stopped,
	/// started represents a state where a child process is running, and the .deploy directory is set up for that child process to run.
	///
	/// to edit the contents of the .deploy directory safely, the child process has to be killed
	Started(Child),
}
static STATE: Lazy<Mutex<State>> = Lazy::new(|| Mutex::new(State::Stopped));

#[derive(Parser, Debug, Clone)]
#[command(name = "deployd")]
#[command(author = "manen")]
struct Cli {
	#[arg(long)]
	/// enabling the systemd option will registed deployd with systemd and enable it
	systemd: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	real_main().await
}
async fn real_main() -> anyhow::Result<()> {
	simple_logger::init_with_level(log::Level::Debug).map_err(|err| anyhow!("{err}"))?;

	let cli = Cli::parse();
	if cli.systemd {
		return systemd();
	}

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
	let resp: deploy_common::Resp = real_message(bin).await.map_err(|err| format!("{err}"));
	match bincode::serialize(&resp) {
		Ok(encoded) => handle!(socket.bin(vec![encoded]).emit("message", [] as [u8; 0])),
		Err(err) => log::error!("{err}"),
	}
}
async fn real_message(Bin(bin): Bin) -> anyhow::Result<()> {
	let messages = bin.into_iter().map(|bin| bincode::deserialize(&bin));

	for res in messages {
		let message: deploy_common::Ctl = res?;

		let launch_new = || {
			log::info!("launching: {message:?}");
			let tmp_dir = reset_dir()?;
			let static_path = match &message {
				Ctl::Executable { path } => {
					let static_path = tmp_dir.join(&path);
					fs::create_dir_all(&tmp_dir)?;
					fs::copy(&path, &static_path)?;
					static_path
				}
				Ctl::DistDir { dir, exec_rel } => {
					let static_path = tmp_dir.join(exec_rel);
					copy_dir::copy_dir(dir, &tmp_dir)?;
					static_path
				}
			};
			Ok::<_, anyhow::Error>(Command::new(&static_path).current_dir(&tmp_dir).spawn()?)
		};

		{
			let mut state = STATE.lock().unwrap();
			let a = (*state).borrow_mut();
			*a = match a {
				State::Stopped => State::Started(launch_new()?),
				State::Started(child) => {
					child.kill()?;
					State::Started(launch_new()?)
				}
			};
		}
	}
	Ok(())
}

/// reset_dir should only be called when the previous child process is killed
pub fn reset_dir() -> anyhow::Result<PathBuf> {
	let path = format!("{}/.deployd", std::env::var("HOME")?);

	if let Ok(_) = fs::metadata(&path) {
		fs::remove_dir_all(&path)?;
	}

	Ok(path.into())
}
