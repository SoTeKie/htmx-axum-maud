use axum::extract::{Path, State};
use axum::routing::get;
use axum::Router;
use maud::{html, Markup};
use uuid::Uuid;

use super::article::Article;
use crate::article::article;
use crate::{base, ServerState};

pub fn router(state: ServerState) -> Router {
	Router::new()
		.route("/", get(index))
		.route("/:id", get(article))
		.with_state(state)
}

async fn article(State(state): State<ServerState>, Path(id): Path<Uuid>) -> Markup {
	let article = article::by_id(state.db, id).await;

	match article {
		Some(article) => article_tmpl(article),
		None => base::error_tmpl(),
	}
}

fn article_tmpl(article: Article) -> Markup {
	html! {
		h2 { (article.title) }
		button hx-get="/articles" hx-trigger="click" hx-target="#root-div" {
			"Go back"
		}
		hr;
		p { (article.text) }
	}
}

async fn index(State(state): State<ServerState>) -> Markup {
	let articles = article::all(state.db).await;

	match articles {
		Some(articles) => index_tmpl(articles),
		None => base::error_tmpl(),
	}
}

fn index_tmpl(articles: Vec<Article>) -> Markup {
	html! {
		@for article in &articles {
			h3 { (article.title) }
			p { (article.short_text()) "..."}
			button hx-get={"/articles/" (article.id)} hx-trigger="click" hx-target="#root-div" {
				"Read more"
			}
			hr;
		}
	}
}
