use super::postgres::DB_POOL;
use std::fs;

pub async fn init_db() {
  for file_name in fs::read_dir("../SQL").unwrap() {
    sqlx::query(
      &fs::read_to_string(format!(
        "../SQL/{}",
        format!("{:?}", file_name.unwrap().file_name()).replace('\"', "")
      ))
      .unwrap(),
    )
    .execute(&*DB_POOL)
    .await
    .unwrap();
  }
}
