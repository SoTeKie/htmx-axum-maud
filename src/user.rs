use axum::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use password_auth::verify_password;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(rename_all = "UPPERCASE")]
pub enum Role {
	Admin,
	Reader,
}

// Quick and dirty implementation, should figure out how to properly
// sync sqlx enum type with postgres enums and make sure it always matches to something
impl Into<Role> for String {
	fn into(self) -> Role {
		match self.as_str() {
			"ADMIN" => Role::Admin,
			_ => Role::Reader,
		}
	}
}

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
	pub id: Uuid,
	pub username: String,
	pub email: String,
	pub role: Role,
	pub first_name: String,
	pub last_name: String,
	password: String,
}

impl User {
	pub fn full_name(&self) -> String {
		format!("{} {}", self.first_name, self.last_name)
	}
}

impl std::fmt::Debug for User {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("User")
			.field("id", &self.id)
			.field("username", &self.username)
			.field("email", &self.email)
			.field("first_name", &self.first_name)
			.field("last_name", &self.last_name)
			.field("password", &"[REDACTED]")
			.finish()
	}
}

impl AuthUser for User {
	type Id = Uuid;

	fn id(&self) -> Self::Id {
		self.id
	}

	fn session_auth_hash(&self) -> &[u8] {
		self.password.as_bytes()
	}
}

#[derive(Clone, Deserialize)]
pub struct Credentials {
	pub username: String,
	pub password: String,
}

#[derive(Debug, Clone)]
pub struct Backend {
	db: PgPool,
}

impl Backend {
	pub fn new(db: PgPool) -> Self {
		Self { db }
	}
}

#[async_trait]
impl AuthnBackend for Backend {
	type User = User;

	type Credentials = Credentials;

	type Error = sqlx::Error;

	async fn authenticate(
		&self,
		creds: Self::Credentials,
	) -> Result<Option<Self::User>, Self::Error> {
		let user = sqlx::query_as!(
			Self::User,
			"SELECT id, username, role, first_name, last_name, email, password FROM users WHERE username = $1", 
			creds.username
		)
		.fetch_optional(&self.db)
		.await?
		.filter(|u| verify_password(creds.password, &u.password).is_ok());

		Ok(user)
	}

	async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
		sqlx::query_as!(
			Self::User,
			"SELECT id, username, role, first_name, last_name, email, password FROM users WHERE id = $1", 
			user_id
		)
		.fetch_optional(&self.db)
		.await
	}
}

pub type AuthSession = axum_login::AuthSession<Backend>;
