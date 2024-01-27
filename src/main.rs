use anyhow::Result;
use article::article_controller;
use axum::routing::get;
use axum::Router;
use axum_login::tower_sessions::cookie::time::Duration;
use axum_login::tower_sessions::{ExpiredDeletion, Expiry, SessionManagerLayer};
use axum_login::AuthManagerLayerBuilder;
use maud::{html, Markup};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tower_http::services::ServeDir;
use tower_sessions_sqlx_store::PostgresStore;
use user::{AuthSession, Backend};

mod article;
mod auth;
mod base;
mod config;
mod user;

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

	sqlx::migrate!().run(&pool).await?;

	let session_store = PostgresStore::new(pool.clone());
	session_store.migrate().await?;

	let deletion_task = tokio::task::spawn(
		session_store
			.clone()
			.continuously_delete_expired(tokio::time::Duration::from_secs(60)),
	);

	let session_layer = SessionManagerLayer::new(session_store)
		.with_expiry(Expiry::OnInactivity(Duration::days(1)));

	let backend = Backend::new(pool.clone());

	let state = ServerState { db: pool };
	let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

	let app = Router::new()
		.route("/", get(index))
		.nest("/articles", article_controller::router(state.clone()))
		.nest("/auth", auth::router(state))
		.layer(auth_layer)
		.nest_service("/static", ServeDir::new("static"));

	let listener = tokio::net::TcpListener::bind(config.server.addr()).await?;

	axum::serve(listener, app.into_make_service()).await?;

	deletion_task.await??;

	Ok(())
}

pub fn user_header(user: Option<user::User>) -> Markup {
	match user {
		Some(user) => html! {
			p { "Welcome " (user.full_name())}
			button hx-get="/auth/logout"  hx-target="#auth-div" { "Logout" }
		},
		None => html!( button hx-get="/auth/login_form"  hx-target="#content-div" { "Login" }),
	}
}

pub fn root_div(user: Option<user::User>) -> Markup {
	html! {
			div #auth-div {
				(user_header(user))
			}
			hr;
			div #content-div {
				p hx-get="/articles" hx-trigger="load" hx-target="#content-div" { "Loading..." }
			}
	}
}

pub async fn index(auth: AuthSession) -> Markup {
	html! {
		(base::header())
		div #root-div {
			(root_div(auth.user))
		}
	}
}
