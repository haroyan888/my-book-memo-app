use axum::{
	extract::{Json, Path, Extension},
	http::StatusCode,
	response::IntoResponse,
};
use serde::{Deserialize, Serialize};

use crate::handler::memo::{create_memo, find_all_memo};
use crate::repos::book::{BookInfo, BookRepository};
use crate::repos::RepositoryError;
use crate::repos::memo::MemoRepository;

pub fn create_book_app<BookRepos: BookRepository, MemoRepos: MemoRepository>(book_repos: &BookRepos, memo_repos: &MemoRepos) -> axum::Router {
	axum::Router::new()
		.route("/", axum::routing::get(find_all_book::<BookRepos>).post(create_book::<BookRepos>))
		.nest(
			"/:isbn_13",
			axum::Router::new()
				.nest(
					"/",
					axum::Router::new().route("/", axum::routing::get(find_book::<BookRepos>).delete(delete_book::<BookRepos>)),
				)
				.nest(
					"/memo",
					axum::Router::new().route("/", axum::routing::get(find_all_memo::<MemoRepos>).post(create_memo::<MemoRepos>)),
				)
		)
		.layer(Extension(book_repos.clone()))
		.layer(Extension(memo_repos.clone()))
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Identifier {
	#[serde(rename = "type")]
	identifier_type: String,
	identifier: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ImageLinks {
	thumbnail: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct VolumeInfoResult {
	title: String,
	description: String,
	authors: Vec<String>,
	publisher: String,
	published_date: String,
	image_links: ImageLinks,
	industry_identifiers: Vec<Identifier>,
}

impl VolumeInfoResult {
	fn to_book_info(&self) -> BookInfo {
		BookInfo {
			isbn_13: self
				.industry_identifiers
				.iter()
				.find(|identifier| identifier.identifier_type == "ISBN_13")
				.unwrap()
				.identifier
				.clone(),
			title: self.title.clone(),
			description: self.description.clone(),
			authors: self.authors.clone(),
			publisher: self.publisher.clone(),
			published_date: self.published_date.clone(),
			image_url: self.image_links.thumbnail.clone(),
		}
	}
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BookInfoResult {
	volume_info: VolumeInfoResult,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SearchBooksResult {
	items: Vec<BookInfoResult>,
}

#[derive(Deserialize)]
struct CreateBook {
	isbn_13: String,
}

fn handle_repository_error(err: RepositoryError) -> StatusCode {
	match err {
		RepositoryError::NotFound(_) => StatusCode::NOT_FOUND,
		RepositoryError::Registered(_) => StatusCode::BAD_REQUEST,
		_ => StatusCode::INTERNAL_SERVER_ERROR,
	}
}

// 登録済みの本を全て返すハンドラ
async fn find_all_book<T: BookRepository>(
	Extension(book_repos): Extension<T>,
) -> Result<impl IntoResponse, StatusCode> {
	let book_info_list = book_repos
		.find_all()
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	Ok((StatusCode::OK, Json(book_info_list)))
}

// 本を検索するハンドラ
async fn find_book<T: BookRepository>(
	Path(isbn_13): Path<String>,
	Extension(book_repos): Extension<T>,
) -> Result<impl IntoResponse, StatusCode> {
	let book_info = book_repos
		.find(&isbn_13)
		.await
		.map_err(handle_repository_error)?;

	Ok((StatusCode::OK, Json(book_info)))
}

// 本を登録するハンドラ
async fn create_book<T: BookRepository>(
	Extension(book_repos): Extension<T>,
	Json(payload): Json<CreateBook>,
) -> Result<impl IntoResponse, StatusCode> {
	if book_repos.find(&payload.isbn_13).await.is_ok() {
		return Err(StatusCode::BAD_REQUEST);
	}
	const URL: &str = "https://www.googleapis.com/books/v1/volumes?q=isbn:";
	let res = reqwest::get(format!("{}{}", URL, &payload.isbn_13))
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
		.text()
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	let search_books_result = serde_json::from_str::<SearchBooksResult>(&res)
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	if search_books_result.items.is_empty() {
		return Err(StatusCode::NOT_FOUND);
	}

	// isbnで一意に検索しているためインデックス0で指定
	let books = search_books_result.items[0].volume_info.to_book_info();

	// Googleがisbn不一致でも良しなに変換してくれるが、ここでははじく
	if payload.isbn_13 != books.isbn_13 {
		return Err(StatusCode::NOT_FOUND);
	}

	let book_info = book_repos
		.create(books)
		.await
		.map_err(handle_repository_error)?;

	Ok((StatusCode::CREATED, Json(book_info)))
}

// 本を削除するハンドラ
async fn delete_book<T: BookRepository>(
	Path(isbn_13): Path<String>,
	Extension(book_repos): Extension<T>,
) -> Result<impl IntoResponse, StatusCode> {
	book_repos
		.delete(&isbn_13)
		.await
		.map_err(handle_repository_error)?;

	Ok((StatusCode::OK, ()))
}
