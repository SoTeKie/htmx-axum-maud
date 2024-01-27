use maud::{html, Markup, DOCTYPE};

pub fn header() -> Markup {
	html! {
		(DOCTYPE)
		head {
			meta charset="utf-8";
			title { "Kieran's Blog" }
			script src="/static/htmx.min.js" {};
		}
		h1 { "Kierans Blog" }
		hr;
	}
}

pub fn error_tmpl() -> Markup {
	html! {
		h1 { "Something went terribly wrong :(" }
	}
}

pub fn error_tmpl_full() -> Markup {
	html! {
		(header())
		h1 { "Something went terribly wrong :(" }
	}
}
