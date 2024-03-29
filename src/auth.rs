use axum::routing::{get, post};
use axum::{Form, Router};
use maud::{html, Markup};

use crate::user::{AuthSession, Credentials};
use crate::{base, ServerState};

pub fn router(state: ServerState) -> Router {
	Router::new()
		.route("/login", post(login))
		.route("/login_form", get(login_form))
		.route("/logout", get(logout))
		.with_state(state)
}

async fn login(mut auth_session: AuthSession, Form(creds): Form<Credentials>) -> Markup {
	match auth_session.authenticate(creds.clone()).await {
		Ok(Some(user)) if auth_session.login(&user).await.is_ok() => crate::index_tmpl(Some(user)),
		_ => base::error_tmpl(),
	}
}

async fn login_form() -> Markup {
	html!(
		form hx-post="/auth/login" hx-target="body" {
			label for="username" {"Username: "}
			input name="username" type="text";

			label for="password" {"Password: "}
			input name="password" type="password";

			input type="submit" value="Login";
		}
	)
}

pub async fn logout(mut auth_session: AuthSession) -> Markup {
	match auth_session.logout().await {
		Ok(_) => crate::index_tmpl(None),
		Err(_) => base::error_tmpl(),
	}
}
