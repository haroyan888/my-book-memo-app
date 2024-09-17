pub mod book;
pub mod memo;

use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
	#[error("Unexpected Error: [{0}]")]
	Unexpected(String),
	#[error("NotFound, isbn is {0}")]
	NotFound(String),
	#[error("Registered, isbn is {0}")]
	Registered(String),
}

pub fn handle_repository_error(err: RepositoryError) -> StatusCode {
	match err {
		RepositoryError::NotFound(_) => StatusCode::NOT_FOUND,
		RepositoryError::Registered(_) => StatusCode::BAD_REQUEST,
		_ => StatusCode::INTERNAL_SERVER_ERROR,
	}
}
