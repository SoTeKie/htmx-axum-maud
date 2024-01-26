use std::net::SocketAddr;

use axum::extract::{Path, State};
use axum::routing::get;
use axum::Router;
use maud::{html, Markup, DOCTYPE};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tower_http::services::ServeDir;
use uuid::Uuid;

mod article;
mod config;

#[derive(Clone)]
pub struct ServerState {
	db: PgPool,
}

#[tokio::main]
async fn main() {
	let config = config::get_config();

	let pool = PgPoolOptions::new()
		.max_connections(10)
		.connect(&config.db.url())
		.await
		.expect("Can't connect to db");

	let state = ServerState { db: pool };

	let app = Router::new()
		.route("/", get(index))
		.route("/articles/:id", get(article))
		.with_state(state)
		.nest_service("/static", ServeDir::new("static"));

	let listener = tokio::net::TcpListener::bind(config.server.addr())
		.await
		.unwrap();

	axum::serve(listener, app.into_make_service())
		.await
		.unwrap()
}

#[axum::debug_handler]
async fn article(State(state): State<ServerState>, Path(id): Path<Uuid>) -> Markup {
	let article = article::by_id(state.db, id)
		.await
		.expect("A server error occured :)");

	html! {
		div #parent-div {
			h2 { (article.title) }
			button hx-get="/" hx-trigger="click" hx-target="#parent-div" hx-swap="outerHTML" {
				"Go back"
			}
			hr;
			p { (article.text) }
		}
	}
}

async fn index(State(state): State<ServerState>) -> Markup {
	let articles = article::all(state.db)
		.await
		.expect("A server error occured :)");

	html! {
		(header())
		div #parent-div {
			@for article in &articles {
				h3 { (article.title) }
				p { (article.short_text()) "..."}
				button hx-get={"/articles/" (article.id)} hx-trigger="click" hx-target="#parent-div" hx-swap="outerHTML" {
					"Read more"
				}
				hr;
			}
		}
	}
}

fn header() -> Markup {
	html! {
		(DOCTYPE)
		head {
			meta charset="utf-8";
			title { "Kieran's Blog" }
			script src="/static/htmx.min.js" {};
		}
	}
}
