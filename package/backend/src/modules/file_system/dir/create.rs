use super::super::super::{
  database::postgres::DB_POOL,
  file_system::{BindType, Group, Tag},
  id::gen_id,
  User,
};
use chrono::Local;

impl Group {
  pub async fn new(
    owner_id: &str,
    name: Option<&str>,
    summary: Option<&str>,
    chmod: Option<[u8; 3]>,
    parent_id: Option<&str>,
    transaction: Option<&mut sqlx::Transaction<'_, sqlx::Postgres>>,
  ) -> Result<Group, anyhow::Error> {
    let created_at = Local::now().timestamp_millis();
    let chmod = chmod.unwrap_or([7, 0, 0]);

    let id = format!("D{}", gen_id(&created_at));

    let parent_id = match parent_id {
      Some(id) => id.to_owned(),
      None => match User::read(owner_id).await {
        Ok(user) => user.root,
        _ => return Err(anyhow::anyhow!("User not found.")),
      },
    };

    let query = sqlx::query(
      r#"
    INSERT INTO "entity_group" (
      id,
      name,
      summary,
      owner_id,
      chmod,
      created_at
    ) VALUES ($1, $2, $3, $4, $5, $6);
    "#,
    )
    .bind(&id)
    .bind(name)
    .bind(summary)
    .bind(owner_id)
    .bind(chmod)
    .bind(created_at);

    match transaction {
      Some(mut tx) => {
        query.execute(&mut **tx).await?;

        Tag::new(
          BindType::Group2Group,
          owner_id,
          &parent_id,
          &id,
          Some("bind"),
          Some(&mut tx),
        )
        .await?;
      }
      _ => {
        let mut tx = DB_POOL.begin().await?;

        query.execute(&mut *tx).await?;

        Tag::new(
          BindType::Group2Group,
          owner_id,
          &parent_id,
          &id,
          Some("bind"),
          Some(&mut tx),
        )
        .await?;

        tx.commit().await?;
      }
    }

    Ok(Group {
      id,
      name: name.map(String::from),
      summary: summary.map(String::from),
      inheritance_id: None,
      owner_id: owner_id.to_string(),
      chmod,
      is_archived: false,
      is_deleted: false,
      created_at,
    })
  }
}
