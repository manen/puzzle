mod shared;
mod to_client;
mod to_server;

pub use shared::Shared;
pub use to_client::ToClient;
pub use to_server::ToServer;

pub const ADDR: &str = "0.0.0.0:4200";
