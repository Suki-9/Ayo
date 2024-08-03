use crate::config::APP_CONFIG;
use futures::executor::block_on;
use once_cell::sync::Lazy;
use sqlx::PgPool;

pub static DB_POOL: Lazy<PgPool> = Lazy::new(|| block_on(init_client()));
pub type PgQuery<'a> = sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>;

// TODO Max Connectionsもconfigで変えられるようにすべきかも...
pub async fn init_client() -> PgPool {
  sqlx::postgres::PgPoolOptions::new()
    .max_connections(20)
    .connect(&format!(
      "postgres://{}:{}@{}:{}/{}",
      &APP_CONFIG.DB_USER,
      &APP_CONFIG.DB_PASSWORD,
      &APP_CONFIG.DB_HOST,
      &APP_CONFIG.DB_PORT,
      &APP_CONFIG.DB_NAME
    ))
    .await
    .unwrap()
}
