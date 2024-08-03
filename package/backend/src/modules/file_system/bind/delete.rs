use super::{
  super::super::database::postgres::{PgQuery, DB_POOL},
  Tag,
};

impl Tag {
  pub async fn delete(
    id: &str,
    transaction: Option<&mut sqlx::Transaction<'_, sqlx::Postgres>>,
  ) -> Result<(), anyhow::Error> {
    let query: PgQuery = sqlx::query(r#"DELETE FROM "Tag" WHERE id = $1;"#).bind(id);

    match transaction {
      Some(tx) => query.execute(&mut **tx).await?,
      _ => query.execute(&*DB_POOL).await?,
    };

    Ok(())
  }
}
