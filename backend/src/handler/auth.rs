use axum::{http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router};
use serde_json::json;

use crate::repos::auth::{AuthSession, Credentials};

pub fn create_auth_app() -> Router<()> {
	Router::new()
		.route("/login", post(login))
		.route("/logout", get(logout))
}

async fn login(
	mut auth_session: AuthSession,
	Json(creds): Json<Credentials>,
) -> Result<impl IntoResponse, impl IntoResponse> {
	let user = match auth_session.authenticate(creds.clone()).await {
		// 成功
		Ok(Some(user)) => user,
		// 認証に失敗した場合
		Ok(None) => {
			return Err((StatusCode::BAD_REQUEST, Json(json!({"message": "Invalid credentials"}))));
		}
		// サーバーエラー
		Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"message": "Internal server error!"})))),
	};
	// セッションの作成
	if auth_session.login(&user).await.is_err() {
		return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"message": "Internal server error!"}))));
	}

	Ok((StatusCode::OK, Json(json!({"message": "success!"}))))
}

async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
	match auth_session.logout().await {
		Ok(_) => Ok((StatusCode::OK, Json(json!({"message": "success!"})))),
		Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"message": "Internal server error!"})))),
	}
}
