use axum::{
	extract::{Json, Path, Extension},
	http::StatusCode,
	response::IntoResponse,
};

use crate::repos::handle_repository_error;
use crate::repos::memo::{CreateMemo, MemoRepository};

pub fn create_memo_app<MemoRepos: MemoRepository>(memo_repos: &MemoRepos) -> axum::Router {
	axum::Router::new().route(
		"/:id",
		axum::routing::get(find_memo::<MemoRepos>).delete(delete_memo::<MemoRepos>)
	)
		.layer(Extension(memo_repos.clone()))
}

// 登録済みのメモを全て返すハンドラ
pub async fn find_all_memo<T: MemoRepository>(
	Path(isbn_13): Path<String>,
	Extension(memo_repos): Extension<T>,
) -> Result<impl IntoResponse, StatusCode> {
	let memo_list = memo_repos
		.find_all(&isbn_13)
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	Ok((StatusCode::OK, Json(memo_list)))
}

// メモを検索するハンドラ
async fn find_memo<T: MemoRepository>(
	Path(id): Path<String>,
	Extension(memo_repos): Extension<T>,
) -> Result<impl IntoResponse, StatusCode> {
	let memo = memo_repos
		.find(&id)
		.await
		.map_err(handle_repository_error)?;

	Ok((StatusCode::OK, Json(memo)))
}

// メモを登録するハンドラ
pub async fn create_memo<T: MemoRepository>(
	Path(isbn_13): Path<String>,
	Extension(memo_repos): Extension<T>,
	Json(payload): Json<CreateMemo>,
) -> Result<impl IntoResponse, StatusCode> {
	let memo = memo_repos
		.create(payload, &isbn_13)
		.await
		.map_err(handle_repository_error)?;

	Ok((StatusCode::CREATED, Json(memo)))
}

// メモを削除するハンドラ
async fn delete_memo<T: MemoRepository>(
	Path(id): Path<String>,
	Extension(memo_repos): Extension<T>,
) -> Result<impl IntoResponse, StatusCode> {
	memo_repos
		.delete(&id)
		.await
		.map_err(handle_repository_error)?;

	Ok((StatusCode::OK, ()))
}
