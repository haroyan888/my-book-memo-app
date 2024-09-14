use super::super::repos::RepositoryError;
use axum::async_trait;
use serde::Serialize;
use sqlx::{Acquire, FromRow, PgPool, Transaction};
use std::{borrow::BorrowMut, sync::Arc};

#[derive(Serialize, Debug, FromRow, PartialEq)]
pub struct BookInfo {
	pub isbn_13: String,
	pub title: String,
	pub authors: Vec<String>,
	pub publisher: String,
	pub published_date: String,
	pub description: String,
	pub image_url: String,
}

#[async_trait]
pub trait BookRepository: Clone + Send + Sync + 'static {
	async fn find(&self, isbn_13: &str) -> Result<BookInfo, RepositoryError>;
	async fn find_all(&self) -> Result<Vec<BookInfo>, RepositoryError>;
	async fn create(&self, payload: BookInfo) -> Result<BookInfo, RepositoryError>;
	async fn delete(&self, isbn_13: &str) -> Result<(), RepositoryError>;
}

#[derive(Clone)]
pub struct BookRepositoryForDB {
	pool: Arc<PgPool>,
}

impl BookRepositoryForDB {
	pub fn new(pool: PgPool) -> Self {
		BookRepositoryForDB {
			pool: Arc::new(pool),
		}
	}

	async fn start_transaction(&self) -> Result<Transaction<'_, sqlx::Postgres>, RepositoryError> {
		self
			.pool
			.begin()
			.await
			.map_err(|err| RepositoryError::Unexpected(err.to_string()))
	}
}

#[async_trait]
impl BookRepository for BookRepositoryForDB {
	async fn find(&self, isbn_13: &str) -> Result<BookInfo, RepositoryError> {
		let mut tx = self.start_transaction().await?;
		let conn = tx
			.acquire()
			.await
			.map_err(|err| RepositoryError::Unexpected(err.to_string()))?;

		let book_info = sqlx::query_as::<_, BookInfo>(
			r#"
				SELECT *, ARRAY (
						SELECT author_name FROM authors WHERE isbn_13 = $1
				) as authors FROM books WHERE isbn_13 = $1;
      "#,
		)
		.bind(isbn_13)
		.fetch_one(conn)
		.await
		.map_err(|err| match err {
			sqlx::Error::RowNotFound => RepositoryError::NotFound(isbn_13.to_string()),
			_ => RepositoryError::Unexpected(err.to_string()),
		})?;

		Ok(book_info)
	}

	async fn find_all(&self) -> Result<Vec<BookInfo>, RepositoryError> {
		let mut tx = self.start_transaction().await?;
		let conn = tx
			.acquire()
			.await
			.map_err(|err| RepositoryError::Unexpected(err.to_string()))?;

		let book_info = sqlx::query_as::<_, BookInfo>(
			r#"
				SELECT *, ARRAY (
						SELECT author_name FROM authors WHERE authors.isbn_13 = books.isbn_13
				) as authors FROM books;
      "#,
		)
		.fetch_all(conn)
		.await
		.map_err(|err| RepositoryError::Unexpected(err.to_string()))?;

		Ok(book_info)
	}

	async fn create(&self, payload: BookInfo) -> Result<BookInfo, RepositoryError> {
		let mut tx = self.start_transaction().await?;
		let conn = tx
			.acquire()
			.await
			.map_err(|err| RepositoryError::Unexpected(err.to_string()))?;

		sqlx::query(r#"INSERT INTO books (isbn_13, title, description, publisher, published_date, image_url) VALUES ($1, $2, $3, $4, $5, $6);"#)
			.bind(&payload.isbn_13)
			.bind(&payload.title)
			.bind(&payload.description)
			.bind(&payload.publisher)
			.bind(&payload.published_date)
			.bind(&payload.image_url)
			.execute(conn.borrow_mut())
			.await
			.map_err(|err| match err.as_database_error() {
					Some(db_err) => match db_err.is_unique_violation() {
							true => RepositoryError::Registered(payload.isbn_13.clone()),
							false => RepositoryError::Unexpected(db_err.to_string()),
					},
					None => RepositoryError::Unexpected(err.to_string())
			})?;

		for author in payload.authors {
			sqlx::query(r#"INSERT INTO authors (isbn_13, author_name) VALUES ($1, $2);"#)
				.bind(&payload.isbn_13)
				.bind(author)
				.execute(conn.borrow_mut())
				.await
				.map_err(|err| RepositoryError::Unexpected(err.to_string()))?;
		}

		tx.commit()
			.await
			.map_err(|err| RepositoryError::Unexpected(err.to_string()))?;

		let book_info = self.find(&payload.isbn_13).await?;

		Ok(book_info)
	}

	async fn delete(&self, isbn_13: &str) -> Result<(), RepositoryError> {
		let mut tx = self
			.start_transaction()
			.await
			.map_err(|err| RepositoryError::Unexpected(err.to_string()))?;
		let conn = tx
			.acquire()
			.await
			.map_err(|err| RepositoryError::Unexpected(err.to_string()))?;

		sqlx::query(r#"DELETE FROM memo WHERE isbn_13 = $1"#)
			.bind(isbn_13)
			.execute(conn.borrow_mut())
			.await
			.map_err(|err| match err {
				sqlx::Error::RowNotFound => RepositoryError::NotFound(isbn_13.to_string()),
				_ => RepositoryError::Unexpected(err.to_string()),
			})?;

		sqlx::query(r#"DELETE FROM authors WHERE isbn_13 = $1"#)
			.bind(isbn_13)
			.execute(conn.borrow_mut())
			.await
			.map_err(|err| match err {
				sqlx::Error::RowNotFound => RepositoryError::NotFound(isbn_13.to_string()),
				_ => RepositoryError::Unexpected(err.to_string()),
			})?;

		sqlx::query(r#"DELETE FROM books WHERE isbn_13 = $1"#)
			.bind(isbn_13)
			.execute(conn.borrow_mut())
			.await
			.map_err(|err| match err {
				sqlx::Error::RowNotFound => RepositoryError::NotFound(isbn_13.to_string()),
				_ => RepositoryError::Unexpected(err.to_string()),
			})?;

		tx.commit()
			.await
			.map_err(|err| RepositoryError::Unexpected(err.to_string()))?;

		Ok(())
	}
}
