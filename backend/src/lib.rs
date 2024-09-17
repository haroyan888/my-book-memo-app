pub mod handler;
pub mod repos;

#[derive(Clone)]
pub struct AppState {
	pub book_repos: repos::book::BookRepositoryForDB,
	pub memo_repos: repos::memo::MemoRepositoryForDB,
}
