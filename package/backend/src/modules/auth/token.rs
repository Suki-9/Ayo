use base64::{
  alphabet,
  engine::{self, general_purpose},
  Engine as _,
};
use radix_fmt::radix_36;
use rand::Rng;
use sha2::{Digest, Sha256};

use super::super::{
  database::redis,
  User,
};

const CUSTOM_ENGINE: engine::GeneralPurpose =
  engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

pub async fn create(user_id: &str, pass: &str) -> Result<String, anyhow::Error> {
  match User::read(user_id).await
  {
    Ok(user) if user.password == Some(CUSTOM_ENGINE.encode(Sha256::digest(pass))) => {
      let token = format!(
        "{:#}",
        radix_36(rand::thread_rng().gen_range(0..(36_i64.pow(5))))
      );

      match redis::create(
        &format!("authority::{}", token),
        user_id,
        86400 * 1000 * 1000,
      ) {
        Ok(_) => Ok(token),
        Err(_) => Err(anyhow::anyhow!("Redis failed to set value.")),
      }
    }
    Ok(_) => Err(anyhow::anyhow!("Incorrect id or password.")),
    Err(e) => {
      println!("{:?}", e);
      Err(anyhow::anyhow!("Read failed in Database."))
    },
  }
}

pub fn verify(token: &str) -> Result<String, anyhow::Error> {
  Ok(redis::read(&format!("authority::{}", token))?)
}

pub fn delete(token: &str) -> Result<(), anyhow::Error> {
  match redis::delete(&format!("authority::{}", token)) {
    Ok(_) => Ok(()),
    Err(_) => Err(anyhow::anyhow!("")),
  }
}
