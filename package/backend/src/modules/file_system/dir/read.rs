use super::{super::super::database::postgres::DB_POOL, Group};
use sqlx::FromRow;

#[derive(FromRow)]
struct ChildId {
  child_id: String,
}

impl Group {
  pub async fn read(id: &str) -> Result<Group, anyhow::Error> {
    Ok(
      sqlx::query_as::<_, Group>(r#"SELECT * FROM "entity_group" WHERE id = $1"#)
        .bind(id)
        .fetch_one(&*DB_POOL)
        .await?,
    )
  }

  pub async fn get_child(id: &str) -> Result<Vec<String>, anyhow::Error> {
    let Ok(root) = Self::read(id).await else {
      return Err(anyhow::anyhow!("Failed to read Directory."));
    };

    let rows = sqlx::query_as::<_, ChildId>(r#"SELECT child_id FROM "tag" WHERE parent_id = $1"#)
      .bind(&root.id)
      .fetch_all(&*DB_POOL)
      .await?;

    Ok(rows.iter().map(|row| row.child_id.to_owned()).collect())
  }
}
