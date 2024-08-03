use super::{
  super::{database::postgres::DB_POOL, file_system::Group, id::gen_id},
  User,
};
use base64::{
  alphabet,
  engine::{self, general_purpose},
  Engine as _,
};
use chrono::Local;
use sha2::{Digest, Sha256};

const CUSTOM_ENGINE: engine::GeneralPurpose =
  engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

impl User {
  pub async fn new(
    password: &str,
    role: &str,
    name: Option<&str>,
    summary: Option<&str>,
  ) -> Result<User, anyhow::Error> {
    let created_at = Local::now().timestamp_millis();
    let id = gen_id(&created_at);

    let mut tx = DB_POOL.begin().await?;

    // Rootのparent_idを 'root' としているがここは改善すべき。
    let root = Group::new(
      &id,
      Some("root"),
      Some("root dir"),
      None,
      Some("root"),
      Some(&mut tx),
    )
    .await?;

    sqlx::query(
      r#"
      INSERT INTO "users" (
        id,
        name,
        summary,
        role,
        password,
        root,
        created_at
      ) VALUES ($1, $2, $3, $4, $5, $6, $7);
      "#,
    )
    .bind(&id)
    .bind(name)
    .bind(summary)
    .bind(role)
    .bind(CUSTOM_ENGINE.encode(Sha256::digest(password)).to_string())
    .bind(&root.id)
    .bind(created_at)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(User {
      id,
      name: name.map(String::from),
      summary: summary.map(String::from),
      role: String::from(role),
      password: None,
      root: root.id,
      created_at,
      deleted_at: None,
      is_deleted: false,
    })
  }
}
