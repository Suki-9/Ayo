use super::api;
use axum::Router;
use tower_http::{
  cors::{Any, CorsLayer},
  services::{ServeDir, ServeFile},
};

pub fn router() -> Router {
  Router::new()
    .nest("/api", api::router())
    .nest_service(
      "/",
      ServeDir::new("../dist").fallback(ServeFile::new("../dist/index.html")),
    )
    .layer(
      CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any),
    )
}
