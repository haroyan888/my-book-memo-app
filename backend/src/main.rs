use sqlx::PgPool;
use std::sync::Arc;
use tower_http::cors;

use backend::handler::{book::create_book_app, memo::create_memo_app};
use backend::repos::{book::BookRepositoryForDB, memo::MemoRepositoryForDB};
use backend::AppState;

fn create_app(state: Arc<AppState>) -> axum::Router {
	axum::Router::new()
		.nest("/book", create_book_app().with_state(state.clone()))
		.nest("/memo", create_memo_app().with_state(state.clone()))
		.layer(
			cors::CorsLayer::new()
				.allow_origin(cors::AllowOrigin::any())
				.allow_headers(cors::AllowHeaders::any())
				.allow_methods(cors::AllowMethods::any()),
		)
}

#[tokio::main]
async fn main() {
	dotenvy::from_path("../.env").ok();

	let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL NOT FOUND");
	let pool = PgPool::connect(&db_url)
		.await
		.expect("failed to create pool");
	let state = Arc::new(AppState {
		book_repos: BookRepositoryForDB::new(pool.clone()),
		memo_repos: MemoRepositoryForDB::new(pool.clone()),
	});

	let host = std::env::var("APP_HOST").expect("APP_HOST NOT FOUND");
	let port = std::env::var("APP_PORT").expect("APP_PORT NOT FOUND");
	let app = create_app(state);
	let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
		.await
		.expect("failed to listen");
	axum::serve(listener, app)
		.await
		.expect("failed to serve app");
}
