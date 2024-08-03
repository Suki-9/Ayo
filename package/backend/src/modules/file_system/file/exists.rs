use super::{super::super::database::postgres::DB_POOL, super::Group, File};

impl File {
  pub async fn exists(id: &str) -> bool {
    matches!(sqlx::query_as::<_, Group>(r#"SELECT * FROM "entity_obj" WHERE id = $1"#)
      .bind(id)
      .fetch_one(&*DB_POOL)
      .await, Ok(row) if row.id == id)
  }
}
