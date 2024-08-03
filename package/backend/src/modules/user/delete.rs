use super::{super::super::database::postgres::DB_POOL, User};
use chrono::Local;

impl User {
  pub async fn delete(
    id: &str,
    transaction: Option<&mut sqlx::Transaction<'_, sqlx::Postgres>>,
  ) -> Result<(), anyhow::Error> {
    let deleted_at = Local::now().timestamp_millis();

    let query = sqlx::query(
      r#"UPDATE "users" SET is_deleted = $1, deleted_at = $2 WHERE id = $3 AND is_deleted = $4;"#,
    )
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
}
