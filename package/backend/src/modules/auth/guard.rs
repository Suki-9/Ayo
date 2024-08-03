use axum::{
  async_trait,
  extract::{FromRequestParts, Request},
  http::{header, StatusCode, request::Parts},
  middleware::Next,
  response::Response,
};
use super::token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserCredential {
  pub user_id: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for UserCredential
where
  S: Send + Sync,
{
  type Rejection = StatusCode;

  async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
    let user = parts
      .extensions
      .get::<Self>()
      .expect("User not found. Did you add auth_middleware?");

    println!("$auth OK");
    Ok(user.clone())
  }
}

pub async fn auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
  println!("access path: {:?}", req.uri().path());
  match req
    .headers()
    .get(header::AUTHORIZATION)
    .and_then(|header| header.to_str().ok())
  {
    Some(token) => {
      let Ok(user_id) = token::verify(token) else {
        println!("Err!");
        return Err(StatusCode::UNAUTHORIZED);
      };

      req.extensions_mut().insert(UserCredential {
        user_id: user_id.to_string(),
      });

      Ok(next.run(req).await)
    }

    None => Err(StatusCode::UNAUTHORIZED),
  }
}
