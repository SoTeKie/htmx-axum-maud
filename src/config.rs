use std::net::IpAddr;

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
	#[clap(env = "DATABASE_URL")]
	pub url: String,
}
