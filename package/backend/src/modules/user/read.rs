use super::{super::database::postgres::DB_POOL, User};

impl User {
  pub async fn read(id: &str) -> Result<User, anyhow::Error> {
    Ok(
      sqlx::query_as::<_, User>(r#"SELECT * FROM "users" WHERE id = $1;"#)
        .bind(id)
        .fetch_one(&*DB_POOL)
        .await?,
    )
  }
}
