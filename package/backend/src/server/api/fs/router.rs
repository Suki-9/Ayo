use super::{modules::auth::guard, FsHandlers};
use axum::{
  extract::DefaultBodyLimit,
  middleware,
  routing::{get, post},
  Router,
};

pub fn routes() -> Router {
  Router::new()
    .route(
      "/",
      post(FsHandlers::create).layer(DefaultBodyLimit::max(256_000_000_000)),
    )
    .route(
      "/:id",
      get(FsHandlers::read)
        .delete(FsHandlers::delete)
        .put(FsHandlers::update),
    )
    .route("/thumbnail/:id", get(FsHandlers::thumbnail))
    .route("/info/:id", get(FsHandlers::info))
    .route_layer(middleware::from_fn(guard::auth))
}
