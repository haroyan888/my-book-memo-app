use axum::{
	extract::{Json, Path, State},
	http::StatusCode,
	response::IntoResponse,
};
use std::sync::Arc;

use crate::repos::handle_repository_error;
use crate::repos::memo::{CreateMemo, MemoRepository};
use crate::AppState;

pub fn create_memo_app() -> axum::Router<Arc<AppState>> {
	axum::Router::new().route("/:id", axum::routing::get(find_memo).delete(delete_memo))
}

// 登録済みのメモを全て返すハンドラ
pub async fn find_all_memo(
	Path(isbn_13): Path<String>,
	State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, StatusCode> {
	let memo_list = state
		.memo_repos
		.find_all(&isbn_13)
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	Ok((StatusCode::OK, Json(memo_list)))
}

// メモを検索するハンドラ
async fn find_memo(
	Path(id): Path<String>,
	State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, StatusCode> {
	let memo = state
		.memo_repos
		.find(&id)
		.await
		.map_err(handle_repository_error)?;

	Ok((StatusCode::OK, Json(memo)))
}

// メモを登録するハンドラ
pub async fn create_memo(
	Path(isbn_13): Path<String>,
	State(state): State<Arc<AppState>>,
	Json(payload): Json<CreateMemo>,
) -> Result<impl IntoResponse, StatusCode> {
	let memo = state
		.memo_repos
		.create(payload, &isbn_13)
		.await
		.map_err(handle_repository_error)?;

	Ok((StatusCode::CREATED, Json(memo)))
}

// メモを削除するハンドラ
async fn delete_memo(
	Path(id): Path<String>,
	State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, StatusCode> {
	state
		.memo_repos
		.delete(&id)
		.await
		.map_err(handle_repository_error)?;

	Ok((StatusCode::OK, ()))
}
