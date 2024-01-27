use article::article_controller;
use axum::routing::get;
use axum::Router;
use maud::{html, Markup};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tower_http::services::ServeDir;
use anyhow::Result;

mod article;
mod base;
mod config;

#[derive(Clone)]
pub struct ServerState {
	db: PgPool,
}

#[tokio::main]
async fn main() -> Result<()> {
	let config = config::get_config();

	let pool = PgPoolOptions::new()
		.max_connections(10)
		.connect(&config.db.url)
		.await?;

	sqlx::migrate!()
		.run(&pool)
		.await?;

	let state = ServerState { db: pool };

	let app = Router::new()
		.route("/", get(index))
		.nest("/articles", article_controller::router(state))
		.nest_service("/static", ServeDir::new("static"));

	let listener = tokio::net::TcpListener::bind(config.server.addr())
		.await?;

	axum::serve(listener, app.into_make_service())
		.await?;

	Ok(())
}

async fn index() -> Markup {
	html! {
		(base::header())
		div #parent-div {
			p hx-get="/articles" hx-trigger="load" hx-target="#parent-div" hx-swap="outerHTML" { "Loading..." }
		}
	}
}
