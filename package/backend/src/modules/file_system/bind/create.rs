use super::super::super::{
  database::postgres::{PgQuery, DB_POOL},
  file_system::{BindType, Tag},
  id::gen_id,
};
use chrono::Local;

impl Tag {
  pub async fn new(
    bind_type: BindType,
    owner_id: &str,
    parent_id: &str,
    child_id: &str,
    value: Option<&str>,
    transaction: Option<&mut sqlx::Transaction<'_, sqlx::Postgres>>,
  ) -> Result<Tag, anyhow::Error> {
    let created_at = Local::now().timestamp_millis();
    let id = gen_id(&created_at);

    let _bind_type = match bind_type {
      BindType::Obj2Obj => "o2o",
      BindType::Group2Obj => "g2o",
      BindType::Group2Group => "g2g",
    };

    let query: PgQuery = sqlx::query(
      r#"
      INSERT INTO "tag" (
        id,
        value,
        owner_id,
        bind_type,
        parent_id,
        child_id,
        created_at
      ) VALUES ($1, $2, $3, $4, $5, $6, $7);
      "#,
    )
    .bind(&id)
    .bind(value)
    .bind(owner_id)
    .bind(_bind_type)
    .bind(parent_id)
    .bind(child_id)
    .bind(created_at);

    match transaction {
      Some(tx) => query.execute(&mut **tx).await?,
      None => query.execute(&(*DB_POOL)).await?,
    };

    Ok(Tag {
      id,
      bind_type,
      owner_id: String::from(owner_id),
      parent_id: String::from(parent_id),
      child_id: String::from(child_id),
      value: value.map(String::from),
      created_at,
    })
  }
}
