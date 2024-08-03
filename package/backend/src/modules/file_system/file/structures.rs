use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize, FromRow)]
pub struct File {
  pub id: String,
  pub inheritance_id: Option<String>,
  pub owner_id: String,

  pub name: Option<String>,
  pub summary: Option<String>,

  #[serde(skip)]
  pub obj_path: String,
  pub mime_type: String,
  pub size: i64,
  pub chmod: [u8; 3],

  pub is_archived: bool,
  pub is_deleted: bool,

  pub created_at: i64,
}
