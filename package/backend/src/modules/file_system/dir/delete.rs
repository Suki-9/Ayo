use super::super::super::{
  database::postgres::{PgQuery, DB_POOL},
  file_system::Group,
};
use chrono::Local;

impl Group {
  pub async fn delete(
    id: &str,
    transaction: Option<&mut sqlx::Transaction<'static, sqlx::Postgres>>,
  ) -> Result<(), anyhow::Error> {
    let deleted_at = Local::now().timestamp_millis();

    let query: PgQuery = sqlx::query(r#"UPDATE "entity_group" SET is_deleted = $1, deleted_at = $2 WHERE id = $3 AND is_deleted = $4;"#)
      .bind(true)
      .bind(deleted_at)
      .bind(id)
      .bind(false);

    match transaction {
      Some(tx) => {
        query.execute(&mut **tx).await?;
      }
      _ => {
        query.execute(&(*DB_POOL)).await?;
      }
    };

    Ok(())
  }

  pub async fn archive(
    id: &str,
    transaction: Option<&mut sqlx::Transaction<'static, sqlx::Postgres>>,
  ) -> Result<(), anyhow::Error> {
    let query: PgQuery = sqlx::query(
      r#"UPDATE "entity_group" SET is_archived = $1 WHERE id = $2 AND is_archived = $3;"#,
    )
    .bind(true)
    .bind(id)
    .bind(false);

    match transaction {
      Some(tx) => {
        query.execute(&mut **tx).await?;
      }
      _ => {
        query.execute(&(*DB_POOL)).await?;
      }
    };

    Ok(())
  }
}
