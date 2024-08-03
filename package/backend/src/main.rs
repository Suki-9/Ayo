mod config;
mod modules;
use modules::database;

#[tokio::main]
async fn main() {
  database::init_db().await;
}
