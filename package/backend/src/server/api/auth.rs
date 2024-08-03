use crate::modules::auth::token;
use axum::{
  extract,
  http::StatusCode,
  response::{IntoResponse, Json},
  routing::post,
  Router,
};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct LoginRequest {
  id: String,
  password: String,
}

pub fn routes() -> Router {
  Router::new().route("/login", post(login))
}

async fn login(extract::Json(payload): extract::Json<LoginRequest>) -> impl IntoResponse {
  match token::create(&payload.id, &payload.password).await {
    Ok(token) => (
      StatusCode::OK,
      Json(json!({
        "i": token,
      })),
    ),
    Err(_) => (
      StatusCode::BAD_REQUEST,
      Json(json!({
        "Err": "failed.",
      })),
    ),
  }
}
