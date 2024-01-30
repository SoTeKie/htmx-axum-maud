use maud::{html, Markup, DOCTYPE};

use crate::user;

pub fn user_header(user: Option<user::User>) -> Markup {
	match user {
		Some(user) => html! {
			p { "Welcome " (user.full_name())}
			button hx-get="/auth/logout"  hx-target="body" { "Logout" }
		},
		None => html!( button hx-get="/auth/login_form"  hx-target="#content-div" { "Login" }),
	}
}

pub fn header(user: Option<user::User>) -> Markup {
	html! {
		(DOCTYPE)
		head {
			meta charset="utf-8";
			title { "Kieran's Blog" }
			script src="/static/htmx.min.js" {};
		}
		h1 { "Kierans Blog" }
		(user_header(user))
		hr;
	}
}

pub fn error_tmpl() -> Markup {
	html! {
		h1 { "Something went terribly wrong :(" }
	}
}
