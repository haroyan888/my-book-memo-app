use axum::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{Acquire, FromRow, PgPool, Postgres, Transaction};
use std::{borrow::BorrowMut, sync::Arc};

use super::RepositoryError;

#[derive(Serialize, Debug, FromRow, PartialEq)]
pub struct Memo {
	pub id: String,
	pub isbn_13: String,
	pub text: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateMemo {
	pub text: String,
}

#[async_trait]
pub trait MemoRepository: Clone + Send + Sync + 'static {
	async fn find(&self, id: &str) -> Result<Memo, RepositoryError>;
	async fn find_all(&self, isbn_13: &str) -> Result<Vec<Memo>, RepositoryError>;
	async fn create(&self, payload: CreateMemo, isbn_13: &str) -> Result<Memo, RepositoryError>;
	async fn delete(&self, id: &str) -> Result<(), RepositoryError>;
}

#[derive(Clone)]
pub struct MemoRepositoryForPg {
	pool: Arc<PgPool>,
}

impl MemoRepositoryForPg {
	pub fn new(pool: PgPool) -> Self {
		MemoRepositoryForPg {
			pool: Arc::new(pool),
		}
	}

	async fn start_transaction(&self) -> Result<Transaction<'_, Postgres>, RepositoryError> {
		self
			.pool
			.begin()
			.await
			.map_err(|err| RepositoryError::Unexpected(err.to_string()))
	}
}

#[async_trait]
impl MemoRepository for MemoRepositoryForPg {
	async fn find(&self, id: &str) -> Result<Memo, RepositoryError> {
		let mut tx = self.start_transaction().await?;
		let conn = tx
			.acquire()
			.await
			.map_err(|err| RepositoryError::Unexpected(err.to_string()))?;

		let memo = sqlx::query_as::<_, Memo>(
			r#"
				SELECT * FROM memo WHERE id = $1;
      "#,
		)
		.bind(id)
		.fetch_one(conn)
		.await
		.map_err(|err| match err {
			sqlx::Error::RowNotFound => RepositoryError::NotFound(id.to_string()),
			_ => RepositoryError::Unexpected(err.to_string()),
		})?;

		Ok(memo)
	}

	async fn find_all(&self, isbn_13: &str) -> Result<Vec<Memo>, RepositoryError> {
		let mut tx = self.start_transaction().await?;
		let conn = tx
			.acquire()
			.await
			.map_err(|err| RepositoryError::Unexpected(err.to_string()))?;

		let memo = sqlx::query_as::<_, Memo>(
			r#"
				SELECT * FROM memo WHERE isbn_13 = $1;
      "#,
		)
		.bind(isbn_13)
		.fetch_all(conn)
		.await
		.map_err(|err| match err {
			sqlx::Error::RowNotFound => RepositoryError::NotFound(isbn_13.to_string()),
			_ => RepositoryError::Unexpected(err.to_string()),
		})?;

		Ok(memo)
	}

	async fn create(&self, payload: CreateMemo, isbn_13: &str) -> Result<Memo, RepositoryError> {
		let mut tx = self.start_transaction().await?;
		let conn = tx
			.acquire()
			.await
			.map_err(|err| RepositoryError::Unexpected(err.to_string()))?;

		// メモを登録したい本が存在しているかを探す
		let book_exist: bool =
			sqlx::query_scalar(r#"SELECT EXISTS(SELECT 1 FROM books WHERE isbn_13 = $1);"#)
				.bind(isbn_13)
				.fetch_one(conn.borrow_mut())
				.await
				.map_err(|err| RepositoryError::Unexpected(err.to_string()))?;
		if !book_exist {
			return Err(RepositoryError::NotFound(isbn_13.to_string()));
		};

		let memo_id = uuid::Uuid::new_v4().to_string();

		let created_memo = sqlx::query_as::<_, Memo>(
			r#"INSERT INTO memo (id, isbn_13, text) VALUES ($1, $2, $3) RETURNING *;"#,
		)
		.bind(&memo_id)
		.bind(isbn_13)
		.bind(&payload.text)
		.fetch_one(conn)
		.await
		.map_err(|err| RepositoryError::Unexpected(err.to_string()))?;

		tx.commit()
			.await
			.map_err(|err| RepositoryError::Unexpected(err.to_string()))?;

		Ok(created_memo)
	}

	async fn delete(&self, id: &str) -> Result<(), RepositoryError> {
		let mut tx = self
			.start_transaction()
			.await
			.map_err(|err| RepositoryError::Unexpected(err.to_string()))?;
		let conn = tx
			.acquire()
			.await
			.map_err(|err| RepositoryError::Unexpected(err.to_string()))?;

		// 削除したいメモが存在しているかを探す
		let memo_exist: bool =
			sqlx::query_scalar(r#"SELECT EXISTS(SELECT 1 FROM memo WHERE id = $1);"#)
				.bind(id)
				.fetch_one(conn.borrow_mut())
				.await
				.map_err(|err| RepositoryError::Unexpected(err.to_string()))?;
		if !memo_exist {
			return Err(RepositoryError::NotFound(id.to_string()));
		};

		sqlx::query(r#"DELETE FROM memo WHERE id = $1 returning *;"#)
			.bind(id)
			.execute(conn)
			.await
			.map_err(|err| RepositoryError::Unexpected(err.to_string()))?;

		tx.commit()
			.await
			.map_err(|err| RepositoryError::Unexpected(err.to_string()))?;

		Ok(())
	}
}
