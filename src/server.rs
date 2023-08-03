use axum::Router;
use std::net::SocketAddr;
use tower_http::services::{ServeDir, ServeFile};

use crate::PUBLIC_DIR;

#[tokio::main]
pub async fn start_server() -> Result<(), anyhow::Error> {
    let serve_dir =
        ServeDir::new(PUBLIC_DIR).not_found_service(ServeFile::new("assets/index.html"));
    let app: Router = Router::new().nest_service("/", serve_dir);
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("serving site on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
