use super::{
  super::super::{
    database::postgres::{PgQuery, DB_POOL},
    id::gen_id,
  },
  File,
};
use chrono::Local;

impl File {
  async fn inheritance(
    inheritance_id: &str,
    owner_id: Option<&str>,
    name: Option<&str>,
    summary: Option<&str>,
    chmod: Option<&[u8; 3]>,
    transaction: Option<&mut sqlx::Transaction<'_, sqlx::Postgres>>,
  ) -> Result<String, anyhow::Error> {
    let Ok(original) = Self::read(inheritance_id).await else {
      return Err(anyhow::anyhow!("Invalid id."));
    };

    if original.is_deleted {
      return Err(anyhow::anyhow!("Deleted Groups cannot be updated."));
    };

    let updated_at = Local::now().timestamp_millis();
    let id = gen_id(&updated_at);
    let name = match name {
      Some(v) => Some(String::from(v)),
      None => original.name,
    };
    let summary = match summary {
      Some(v) => Some(String::from(v)),
      None => original.summary,
    };
    let chmod = chmod.unwrap_or(&[7, 0, 0]);

    let query: PgQuery = sqlx::query(
      r#"
        INSERT INTO "entity_group" (
          id,
          inheritance_id,
          name,
          summary,
          owner_id,
          obj_path,
          mime_type,
          size,
          chmod,
          created_at
          is_archived,
        ) VALUES ($1, $2, $3, $4, $5, $6);
        "#,
    )
    .bind(&id)
    .bind(inheritance_id)
    .bind(name)
    .bind(summary)
    .bind(owner_id)
    .bind(original.obj_path)
    .bind(original.size)
    .bind(chmod)
    .bind(original.created_at)
    .bind(original.is_archived);

    match transaction {
      Some(tx) => {
        query.execute(&mut **tx).await?;
      }
      _ => {
        query.execute(&(*DB_POOL)).await?;
      }
    };

    Ok(id)
  }

  pub async fn rename(
    id: &str,
    name: &str,
    transaction: Option<&mut sqlx::Transaction<'_, sqlx::Postgres>>,
  ) -> Result<String, anyhow::Error> {
    Self::inheritance(&id, None, Some(name), None, None, transaction).await
  }

  pub async fn update_summary(
    id: &str,
    summary: &str,
    transaction: Option<&mut sqlx::Transaction<'_, sqlx::Postgres>>,
  ) -> Result<String, anyhow::Error> {
    Self::inheritance(&id, None, None, Some(summary), None, transaction).await
  }

  pub async fn chmod(
    id: &str,
    chmod: &[u8; 3],
    transaction: Option<&mut sqlx::Transaction<'_, sqlx::Postgres>>,
  ) -> Result<String, anyhow::Error> {
    Self::inheritance(&id, None, None, None, Some(&chmod), transaction).await
  }
}
