mod config;
mod modules;
mod server;

use modules::database;

#[tokio::main]
async fn main() {
  database::init_db().await;
  server::init_server().await;
}
