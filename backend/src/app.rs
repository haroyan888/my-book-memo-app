use axum::http;
use axum_login::{
	login_required,
	tower_sessions::{ExpiredDeletion, Expiry, SessionManagerLayer},
	AuthManagerLayerBuilder,
};
use sqlx::PgPool;
use tower_http::cors;
use tokio::{signal, task::AbortHandle};
use tower_sessions::cookie::Key;
use tower_sessions_sqlx_store::PostgresStore;
use time::Duration;

use crate::handler::{
	book::create_book_app,
	memo::create_memo_app,
	auth::create_auth_app
};
use crate::repos::{
	book::{BookRepositoryForPg, BookRepository},
	memo::{MemoRepositoryForPg, MemoRepository},
	auth::AuthRepositoryForPg,
};

pub struct App {
	db: PgPool,
}

impl App {
	pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
		let db_connection_str = std::env::var("DATABASE_URL").expect("DATABASE_URL is not defined");
		let db = PgPool::connect(&db_connection_str)
			.await?;
		sqlx::migrate!("../migrations").run(&db).await?;

		Ok(Self { db })
	}

	pub async fn serve(self) -> Result<(), Box<dyn std::error::Error>> {
		let session_store = PostgresStore::new(self.db.clone());
		session_store.migrate().await?;

		let deletion_task = tokio::task::spawn(
			session_store
				.clone()
				.continuously_delete_expired(tokio::time::Duration::from_secs(60)),
		);

		// Generate a cryptographic key to sign the session cookie.
		let key = Key::generate();

		let session_layer = SessionManagerLayer::new(session_store)
			.with_secure(false)
			.with_expiry(Expiry::OnInactivity(Duration::days(1)))
			.with_signed(key);

		let backend = AuthRepositoryForPg::new(self.db.clone());
		let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

		let book_repos = BookRepositoryForPg::new(self.db.clone());
		let memo_repos = MemoRepositoryForPg::new(self.db.clone());

		let host = std::env::var("APP_HOST").expect("APP_HOST is not defined");
		let port = std::env::var("APP_PORT").expect("APP_PORT is not defined");

		let app = create_app(book_repos, memo_repos)
			.route_layer(login_required!(AuthRepositoryForPg))
			.merge(create_auth_app())
			.layer(auth_layer)
			.layer(
				cors::CorsLayer::new()
					.allow_origin("http://localhost:5173".parse::<http::HeaderValue>().unwrap())
					.allow_headers([
						http::header::AUTHORIZATION,
						http::header::ACCEPT,
						http::header::CONTENT_TYPE
					])
					.allow_methods([
						http::method::Method::GET,
						http::method::Method::POST,
						http::method::Method::DELETE,
					])
					.allow_credentials(true),
			);

		let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
			.await
			.expect("failed to listen");

		// Ensure we use a shutdown signal to abort the deletion task.
		axum::serve(listener, app.into_make_service())
			.with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle()))
			.await?;

		deletion_task.await.ok();

		Ok(())
	}
}

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
}

async fn shutdown_signal(deletion_task_abort_handle: AbortHandle) {
	let ctrl_c = async {
		signal::ctrl_c()
			.await
			.expect("failed to install Ctrl+C handler");
	};

	#[cfg(unix)]
	let terminate = async {
		signal::unix::signal(signal::unix::SignalKind::terminate())
			.expect("failed to install signal handler")
			.recv()
			.await;
	};

	#[cfg(not(unix))]
	let terminate = std::future::pending::<()>();

	tokio::select! {
        _ = ctrl_c => { deletion_task_abort_handle.abort() },
        _ = terminate => { deletion_task_abort_handle.abort() },
    }
}
