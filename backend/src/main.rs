use sqlx::PgPool;
use tower_http::cors;
use tokio::signal;

use backend::handler::{book::create_book_app, memo::create_memo_app};
use backend::repos::{book::{BookRepositoryForPg, BookRepository}, memo::{MemoRepositoryForPg, MemoRepository}};

fn create_app<BookRepos, MemoRepos>(book_repos: BookRepos, memo_repos: MemoRepos) -> axum::Router
where
	BookRepos: BookRepository,
	MemoRepos: MemoRepository,
{
	axum::Router::new()
		.nest(
			"/book",
			create_book_app(&book_repos, &memo_repos)
		)
		.nest(
			"/memo",
			create_memo_app(&memo_repos)
		)
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
	let book_repos = BookRepositoryForPg::new(pool.clone());
	let memo_repos = MemoRepositoryForPg::new(pool.clone());

	let host = std::env::var("APP_HOST").expect("APP_HOST NOT FOUND");
	let port = std::env::var("APP_PORT").expect("APP_PORT NOT FOUND");
	let app = create_app(book_repos, memo_repos);
	let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
		.await
		.expect("failed to listen");

	axum::serve(listener, app)
		.with_graceful_shutdown(async { signal::ctrl_c().await.unwrap() })
		.await
		.expect("failed to serve app");

	println!("exit");
}
