pub mod book;
pub mod memo;
pub mod repos;

#[derive(Clone)]
pub struct AppState {
	pub book_repos: book::repos::BookRepositoryForDB,
	pub memo_repos: memo::repos::MemoRepositoryForDB,
}
