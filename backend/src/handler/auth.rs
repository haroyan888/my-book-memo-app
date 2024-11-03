use axum::{
	http::StatusCode,
	response::{IntoResponse, Redirect},
	routing::{get, post},
	Json,
	Form,
	body::Body,
	Router
};
use serde_json::json;

use crate::repos::auth::{AuthSession, Credentials};

pub fn create_auth_app() -> Router<()> {
	Router::new()
		.route("/create-account", post(create_account))
		.route("/login", post(login))
		.route("/account", get(get_account).post(create_account).delete(delete_account))
		.route("/logout", get(logout))
}

async fn create_account(
	mut auth_session: AuthSession,
	Form(creds): Form<Credentials>,
) -> Result<impl IntoResponse, impl IntoResponse> {
	let find_account_res = auth_session
		.backend
		.find_account(&creds.email)
		.await
		.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"message": "サーバーエラー"}))))?;

	if find_account_res.is_some() {
		return Err((StatusCode::BAD_REQUEST, Json(json!({"message": "すでにアカウントが存在します"}))));
	}

	if auth_session.backend.create_account(creds.clone()).await.is_err() {
		return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"message": "サーバーエラー"}))));
	}

	let user = match auth_session.authenticate(creds.clone()).await {
		// 成功
		Ok(Some(user)) => user,
		// 認証に失敗した場合
		Ok(None) => {
			return Err((StatusCode::BAD_REQUEST, Json(json!({"message": "Invalid credentials"}))));
		}
		// サーバーエラー
		Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"message": "サーバーエラー"})))),
	};

	// セッションの作成
	if auth_session.login(&user).await.is_err() {
		return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"message": "サーバーエラー"}))));
	}

	Ok(Redirect::to(&creds.next))
}

async fn login(
	mut auth_session: AuthSession,
	Form(creds): Form<Credentials>,
) -> Result<impl IntoResponse, impl IntoResponse> {
	let find_account_res = auth_session
		.backend
		.find_account(&creds.email)
		.await
		.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"message": "サーバーエラー"}))))?;

	if find_account_res.is_none() {
		return Err((StatusCode::BAD_REQUEST, Json(json!({"message": "アカウントが見つかりませんでした"}))));
	}

	let user = match auth_session.authenticate(creds.clone()).await {
		// 成功
		Ok(Some(user)) => user,
		// 認証に失敗した場合
		Ok(None) => {
			return Err((StatusCode::BAD_REQUEST, Json(json!({"message": "認証に失敗しました"}))));
		}
		// サーバーエラー
		Err(err) => {
			println!("{}", err);
			return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"message": "サーバーエラー"}))));
		}
	};

	// セッションの作成
	if auth_session.login(&user).await.is_err() {
		return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"message": "サーバーエラー"}))));
	}

	Ok(Redirect::to(&creds.next))
}

async fn get_account(
	auth_session: AuthSession
) -> impl IntoResponse {
	let user = auth_session.user.clone();
	if user.is_none() {
		return (StatusCode::UNAUTHORIZED, Body::empty()).into_response();
	}

	(StatusCode::OK, Json(json!({"email": user.unwrap().email})),).into_response()
}

async fn delete_account(
	mut auth_session: AuthSession
) -> impl IntoResponse {
	if auth_session.user.is_none() {
		return StatusCode::UNAUTHORIZED
	}

	let user_id = auth_session.user.clone().unwrap().id;

	if let Err(e) = auth_session.logout().await {
		println!("{}", e);
		return StatusCode::INTERNAL_SERVER_ERROR;
	}

	if let Err(e) = auth_session.backend.delete_account(&user_id).await {
		println!("{}", e);
		return StatusCode::INTERNAL_SERVER_ERROR;
	}

	StatusCode::NO_CONTENT
}

async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
	match auth_session.logout().await {
		Ok(_) => Ok((StatusCode::OK, Json(json!({"message": "成功"})))),
		Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"message": "サーバーエラー"})))),
	}
}
