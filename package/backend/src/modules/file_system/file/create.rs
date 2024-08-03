use crate::{
  config::APP_CONFIG,
  modules::{
    database::postgres::{PgQuery, DB_POOL},
    file_system::{BindType, File, Tag},
    id::gen_id,
    User,
  },
};
use chrono::Local;
use tokio::{fs, io::AsyncWriteExt};

impl File {
  pub async fn new(
    owner_id: &str,
    name: Option<&str>,
    summary: Option<&str>,
    mime_type: &str,
    chmod: Option<[u8; 3]>,
    parent_id: Option<&str>,
    data: Vec<u8>,
    transaction: Option<&mut sqlx::Transaction<'_, sqlx::Postgres>>,
  ) -> Result<File, anyhow::Error> {
    let created_at = Local::now().timestamp_millis();
    let id = format!("F{}", gen_id(&created_at));

    let obj_path = format!("{}/{}", APP_CONFIG.STATIC_FILE_DIR, id);
    let chmod = chmod.unwrap_or([7, 0, 0]);

    let size = Self::write_file(data, &obj_path).await?;

    // TODO 可読性上げろ
    let parent_id = match parent_id {
      // TODO 存在確認をする。
      Some(id) => id.to_string(),
      None => match User::read(owner_id).await {
        Ok(user) => user.root,
        Err(e) => {
          println!("{:?}", e);
          // sqlxではu32型は使えないのでi64にキャストする。
          return Err(anyhow::anyhow!("DB error"));
        }
      },
    };

    let query: PgQuery = sqlx::query(
      r#"
      INSERT INTO "entity_obj" (
        id,
        name,
        summary,
        owner_id,
        obj_path,
        mime_type,
        size,
        chmod,
        created_at
      ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9);
      "#,
    )
    .bind(&id)
    .bind(name)
    .bind(summary)
    .bind(owner_id)
    .bind(&obj_path)
    .bind(mime_type)
    .bind(size)
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

    Ok(File {
      id,
      inheritance_id: None,
      owner_id: String::from(owner_id),
      name: name.map(String::from),
      summary: summary.map(String::from),
      obj_path,
      mime_type: String::from(mime_type),
      size,
      chmod,
      is_archived: false,
      is_deleted: false,
      created_at,
    })
  }

  async fn write_file(data: Vec<u8>, obj_path: &str) -> Result<i64, anyhow::Error> {
    let mut file = fs::File::create(obj_path).await?;

    file.write_all(&data).await?;

    Ok(i64::try_from(u32::try_from(data.len())?)?)
  }
}
