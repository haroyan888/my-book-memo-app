use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use axum_login::AuthUser;

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct User {
	pub id: String,
	pub username: String,
	pub password: String,
}

impl std::fmt::Debug for User {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("User")
			.field("id", &self.id)
			.field("username", &self.username)
			.field("password", &"[redacted]")
			.finish()
	}
}

impl AuthUser for User {
	type Id = String;

	fn id(&self) -> Self::Id {
		self.id.clone()
	}

	fn session_auth_hash(&self) -> &[u8] {
		self.password.as_bytes()
	}
}
