use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

pub async fn start(static_dir: String, port: u16) -> Result<(), Box<dyn std::error::Error>> {
  let serve_dir = ServeDir::new(static_dir.clone());
  let app = Router::new().fallback_service(serve_dir);
  let addr = SocketAddr::from(([127, 0, 0, 1], port));
  let listener = TcpListener::bind(addr).await?;

  println!("🚀 Serving '{}' at http://{}", static_dir, addr);

  axum::serve(listener, app).await?;

  Ok(())
}
