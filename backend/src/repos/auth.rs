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
pub struct AuthRepository {
	db: PgPool,
}

impl AuthRepository {
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
impl AuthnBackend for AuthRepository {
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

pub type AuthSession = axum_login::AuthSession<AuthRepository>;
