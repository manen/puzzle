use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub const ADDR: &str = "127.0.0.1:6789";

#[derive(Parser, Debug, Clone, Serialize, Deserialize)]
#[command(name = "deployctl")]
#[command(author = "manen")]
pub enum Ctl {
	Executable { path: PathBuf },
	DistDir { dir: PathBuf, exec_rel: PathBuf },
}

pub type Resp = Result<(), String>;
