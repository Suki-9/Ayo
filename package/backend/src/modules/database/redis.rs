use crate::config::APP_CONFIG;
use once_cell::sync::Lazy;
use redis::{Client, Commands, FromRedisValue, RedisError, ToRedisArgs};

pub static REDIS_CLIENT: Lazy<Client> = Lazy::new(|| {
  redis::Client::open(format!(
    "redis://{}:{}/",
    &APP_CONFIG.REDIS_HOST, &APP_CONFIG.REDIS_PORT
  ))
  .expect("redis connect error")
});

pub fn create<V: ToRedisArgs>(key: &str, value: V, expires: u64) -> Result<(), RedisError> {
  match REDIS_CLIENT.get_connection() {
    Ok(mut con) => con.set_ex(format!("{}::{}", &APP_CONFIG.APP_HOST, key), value, expires),
    Err(e) => Err(e),
  }
}

pub fn read<V: FromRedisValue>(key: &str) -> Result<V, RedisError> {
  match REDIS_CLIENT.get_connection() {
    Ok(mut con) => con.get(format!("{}::{}", &APP_CONFIG.APP_HOST, key)),
    Err(e) => Err(e),
  }
}

pub fn delete(key: &str) -> Result<(), RedisError> {
  match REDIS_CLIENT.get_connection() {
    Ok(mut con) => con.del(format!("{}::{}", &APP_CONFIG.APP_HOST, key)),
    Err(e) => Err(e),
  }
}
