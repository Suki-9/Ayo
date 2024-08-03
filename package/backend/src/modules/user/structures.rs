use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize, Clone, FromRow)]
pub struct User {
  pub id: String,
  pub name: Option<String>,
  pub summary: Option<String>,
  pub role: String,
  #[serde(skip)]
  pub password: Option<String>,
  pub root: String,
  pub created_at: i64,
  pub deleted_at: Option<i64>,
  pub is_deleted: bool,
}
