use once_cell::sync::Lazy;
use serde_derive::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  pub APP_HOST: String,
  pub APP_PORT: i32,
  pub DB_HOST: String,
  pub DB_PORT: String,
  pub DB_USER: String,
  pub DB_PASSWORD: String,
  pub DB_NAME: String,
  pub REDIS_HOST: String,
  pub REDIS_PORT: String,
  pub STATIC_FILE_DIR: String,
}

pub static APP_CONFIG: Lazy<Config> = Lazy::new(|| {
  match serde_yaml::from_str(
    &std::fs::read_to_string("../../configs/backend.default.yaml").unwrap(),
  ) {
    Ok(c) => c,
    Err(_) => panic!("config file open faild."),
  }
});
