use axum_login::{AuthnBackend, UserId};
use password_auth::verify_password;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tokio::task;
use axum::async_trait;

use crate::entity::user::User;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Credentials {
	pub username: String,
	pub password: String,
}

#[derive(Debug, Clone)]
pub struct AuthRepositoryForPg {
	db: PgPool,
}

impl AuthRepositoryForPg {
	pub fn new(db: PgPool) -> Self {
		Self { db }
	}
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error(transparent)]
	Sqlx(#[from] sqlx::Error),

	#[error(transparent)]
	TaskJoin(#[from] task::JoinError),
}

#[async_trait]
impl AuthnBackend for AuthRepositoryForPg {
	type User = User;
	type Credentials = Credentials;
	type Error = Error;

	async fn authenticate(
		&self,
		creds: Self::Credentials,
	) -> Result<Option<Self::User>, Self::Error> {
		let user: Option<Self::User> = sqlx::query_as("select * from users where username = $1")
			.bind(&creds.username)
			.fetch_optional(&self.db)
			.await?;

		task::spawn_blocking(|| {
			Ok(user.filter(|user| verify_password(creds.password, &user.password).is_ok()))
		})
			.await?
	}

	async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
		let user = sqlx::query_as("select * from users where id = $1")
			.bind(user_id)
			.fetch_optional(&self.db)
			.await?;

		Ok(user)
	}
}

impl AuthRepositoryForPg {
	pub async fn find_account(&self, username: &str) -> Result<Option<User>, Error> {
		let user: Option<User> = sqlx::query_as("select * from users where username = $1")
			.bind(username)
			.fetch_optional(&self.db)
			.await?;

		Ok(user)
	}
	pub async fn create_account(
		&self,
		credentials: Credentials
	) -> Result<User, Error> {
		let id = uuid::Uuid::new_v4().to_string();
		let hashed_password = password_auth::generate_hash(&credentials.password);
		// ユーザを作成
		let user: User = sqlx::query_as("insert into users(id, username, password) values ($1, $2, $3) returning *;")
			.bind(id)
			.bind(&credentials.username)
			.bind(hashed_password)
			.fetch_one(&self.db)
			.await?;

		Ok(user)
	}
}

pub type AuthSession = axum_login::AuthSession<AuthRepositoryForPg>;
