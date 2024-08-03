use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Group {
  pub id: String,
  pub inheritance_id: Option<String>,
  pub owner_id: String,

  pub name: Option<String>,
  pub summary: Option<String>,

  pub chmod: [u8; 3],

  pub is_archived: bool,
  pub is_deleted: bool,

  pub created_at: i64,
}
