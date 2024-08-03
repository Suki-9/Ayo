use serde_json::json;
use std::collections::HashMap;

use axum::{
  extract::Path,
  http::StatusCode,
  response::{IntoResponse, Json},
  routing::get,
  Router,
};

pub fn routes() -> Router {
  Router::new().nest(
    "/tag",
    Router::new().route("/list/:attr", get(search_tag_list)),
  )
}

async fn search_tag_list(Path(params): Path<HashMap<String, String>>) -> impl IntoResponse {
  let Some(attr) = params.get("attr") else {
    return Err(StatusCode::BAD_REQUEST);
  };

  Ok((StatusCode::OK, Json(json!("{}"))))
}
