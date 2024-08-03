use super::*;
use axum::Router;

pub fn router() -> Router {
  Router::new()
    .nest(
      "/",
      Router::new()
        .nest("/user", user::routes())
        .nest("/auth", auth::routes())
        .nest("/fs", fs::routes())
        .nest("/search", search::routes()),
    )
    .nest("/", Router::new())
}
