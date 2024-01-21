use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct Article {
	pub id: Uuid,
	pub title: String,
	pub text: String,
}

impl Article {
	pub fn short_text(&self) -> String {
		self.text[..250].to_string()
	}
}

pub async fn all(db: PgPool) -> Option<Vec<Article>> {
	sqlx::query_as!(Article, "SELECT id, title, content as text FROM articles")
		.fetch_all(&db)
		.await
		.ok()
}

pub async fn by_id(db: PgPool, id: Uuid) -> Option<Article> {
	sqlx::query_as!(
		Article,
		"SELECT id, title, content as text FROM articles WHERE id = $1",
		id
	)
	.fetch_one(&db)
	.await
	.ok()
}
