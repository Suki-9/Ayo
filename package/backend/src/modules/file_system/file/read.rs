use super::{super::super::database::postgres::DB_POOL, File};
use tokio::fs;

impl File {
  pub async fn read(id: &str) -> Result<File, anyhow::Error> {
    Ok(
      sqlx::query_as::<_, File>(r#"SELECT * FROM "entity_obj" WHERE id = $1"#)
        .bind(id)
        .fetch_one(&*DB_POOL)
        .await?,
    )
  }

  async fn read_file(path: &str) -> Result<Vec<u8>, anyhow::Error> {
    Ok(fs::read(path).await?)
  }
}
