use super::{super::super::database::postgres::DB_POOL, File};
use chrono::Local;
use tokio::fs;

impl File {
  pub async fn delete(
    id: &str,
    transaction: Option<&mut sqlx::Transaction<'_, sqlx::Postgres>>,
  ) -> Result<(), anyhow::Error> {
    let deleted_at = Local::now().timestamp_millis();

    let query = sqlx::query(r#"UPDATE "entity_obj" SET is_deleted = $1, deleted_at = $2 WHERE id = $3 AND is_deleted = $4;"#)
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

  pub async fn restore(
    id: &str,
    transaction: Option<&mut sqlx::Transaction<'_, sqlx::Postgres>>,
  ) -> Result<(), anyhow::Error> {
    let expire = Local::now().timestamp_millis() - 2592000000;

    let query =
      sqlx::query(r#"UPDATE "entity_obj" SET is_deleted = $1, WHERE id = $2 AND deleted_at > $3;"#)
        .bind(false)
        .bind(id)
        .bind(expire);

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
    transaction: Option<&mut sqlx::Transaction<'_, sqlx::Postgres>>,
  ) -> Result<(), anyhow::Error> {
    let query = sqlx::query(
      r#"UPDATE "entity_obj" SET is_archived = $1 WHERE id = $2 AND is_archived = $3;"#,
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

  async fn delete_obj(path: &str) -> Result<(), &'static str> {
    match fs::remove_file(path).await {
      Err(e) => {
        println!("{:?}", e);
        Err("Failed to delete file.")
      }
      _ => Ok(()),
    }
  }
}
