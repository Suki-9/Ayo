use crate::modules::{api::response::ApiResult, auth::token, database::postgres::DB_POOL, User};
use axum::{
  extract,
  extract::Request,
  http::{header, StatusCode},
  response::{IntoResponse, Json},
  routing::post,
  Router,
};
use chrono::Local;
use serde::Deserialize;
use serde_json::json;

pub fn routes() -> Router {
  Router::new().route(
    "/",
    post(create_user)
      .get(show_user)
      .put(update_user)
      .delete(delete_user),
  )
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct UserCreateRequest {
  name: Option<String>,
  password: String,
  role: String,
}

async fn create_user(
  extract::Json(payload): extract::Json<UserCreateRequest>,
) -> ApiResult<impl IntoResponse> {
  let Ok(user) = User::new(
    &payload.password,
    &payload.role,
    payload.name.as_deref(),
    None,
  )
  .await
  else {
    return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
  };

  let token = match token::create(&user.id, &payload.password).await {
    Ok(token) => token,
    Err(e) => {
      println!("{:?}", e);
      return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    }
  };

  Ok(
    (
      StatusCode::OK,
      Json(json!({
        "id": user.id,
        "name": user.name,
        "createAt": user.created_at,
        "i": token,
        "root": user.root,
      })),
    )
      .into_response(),
  )
}

async fn show_user(req: Request) -> ApiResult<impl IntoResponse> {
  let Some(token) = req
    .headers()
    .get(header::AUTHORIZATION)
    .and_then(|header| header.to_str().ok())
  else {
    return Ok(StatusCode::UNAUTHORIZED.into_response());
  };

  let user_id = token::verify(token)?;
  let user = User::read(&user_id).await?;

  Ok((StatusCode::OK, Json(json!(user))).into_response())
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct UserUpdateRequest {}

async fn update_user(
  extract::Json(_payload): extract::Json<UserUpdateRequest>,
) -> impl IntoResponse {
  (
    StatusCode::NOT_FOUND,
    Json(json!({
      "Err": "TODO",
    })),
  )
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct UserDeleteRequest {
  i: String,
}

async fn delete_user(
  extract::Json(payload): extract::Json<UserDeleteRequest>,
) -> ApiResult<impl IntoResponse> {
  let success_res = (
    StatusCode::OK,
    Json(json!({
      "Ok": "Delete Success.",
    })),
  );

  let err_res = (
    StatusCode::BAD_REQUEST,
    Json(json!({
      "Err": "Delete failed.",
    })),
  );

  match token::verify(&payload.i) {
    Ok(user_id) => {
      let deleted_at = Local::now().timestamp_millis();

      let _ = sqlx::query(
        r#"
        UPDATE "users" SET "deleted_at" = $1, "is_deleted" = $2 WHERE "id" = $3;
      "#,
      )
      .bind(deleted_at)
      .bind(true)
      .bind(&user_id)
      .fetch_one(&*DB_POOL)
      .await?;

      User::delete(&user_id, None).await?;

      Ok(success_res.into_response())
    }
    _ => Ok(err_res.into_response()),
  }
}
