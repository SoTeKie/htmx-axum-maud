use std::net::{IpAddr, SocketAddr};

use clap::Parser;

pub fn get_config() -> FullConfig {
	dotenvy::dotenv().ok();
	FullConfig {
		server: ServerConfig::parse(),
		db: DbConfig::parse(),
	}
}

pub struct FullConfig {
	pub server: ServerConfig,
	pub db: DbConfig,
}

#[derive(Debug, Parser)]
pub struct ServerConfig {
	#[clap(env = "SERVER_HOST")]
	pub host: IpAddr,
	#[clap(env = "SERVER_PORT")]
	pub port: u16,
}

#[derive(Debug, Parser)]
pub struct DbConfig {
	#[clap(env = "DATABASE_PORT")]
	pub port: u16,
	#[clap(env = "DATABASE_USER")]
	pub user: String,
	#[clap(env = "DATABASE_PASSWORD")]
	pub password: String,
}

impl ServerConfig {
	pub fn addr(&self) -> SocketAddr {
		SocketAddr::from((self.host, self.port))
	}
}

impl DbConfig {
	pub fn url(&self) -> String {
		format!(
			"postgres://{}:{}@localhost:{}/blog",
			self.user, self.password, self.port
		)
	}
}
