use backend::app::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	dotenvy::dotenv().ok();
	App::new().await?.serve().await
}
