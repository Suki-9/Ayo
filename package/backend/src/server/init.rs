use super::{super::config::APP_CONFIG, router};
use tokio::net::TcpListener;

pub async fn init_server() {
  axum::serve(
    TcpListener::bind(format!("0.0.0.0:{}", APP_CONFIG.APP_PORT))
      .await
      .unwrap(),
    router(),
  )
  .await
  .unwrap();
}
