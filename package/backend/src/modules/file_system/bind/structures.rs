use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum BindType {
  Obj2Obj,
  Group2Obj,
  Group2Group,
}

#[derive(Deserialize, Serialize)]
pub struct Tag {
  pub id: String,
  pub bind_type: BindType,

  pub owner_id: String,

  pub parent_id: String,
  pub child_id: String,

  pub value: Option<String>,
  pub created_at: i64,
}
