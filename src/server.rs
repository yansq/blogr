use std::{net::SocketAddr, time::Duration};

use crate::build::rebuild_site;
use crate::CONTENT_DIR;
use crate::PUBLIC_DIR;

pub async fn start_server() -> Result<(), anyhow::Error> {
    use axum::Router;
    use tower_http::services::{ServeDir, ServeFile};

    info!("Start server...");
    let serve_dir =
        ServeDir::new(PUBLIC_DIR).not_found_service(ServeFile::new("public/assets/404.html"));
    let app: Router = Router::new().nest_service("/", serve_dir);
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("serving site on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

pub async fn hot_update() {
    use hotwatch::{notify::event::ModifyKind, Event, EventKind};

    let mut hotwatch = hotwatch::Hotwatch::new().expect("hotwatch failed to initialize!");
    hotwatch
        .watch(CONTENT_DIR, move |event: Event| match event.kind {
            EventKind::Modify(m) => match m {
                ModifyKind::Metadata(_) => {}
                _ => {
                    rebuild_site(CONTENT_DIR, PUBLIC_DIR).expect("Rebuilding site failed");
                }
            },
            EventKind::Create(_) | EventKind::Remove(_) => {
                info!("Rebuilding site");
                rebuild_site(CONTENT_DIR, PUBLIC_DIR).expect("Rebuilding site failed");
            }
            _ => {}
        })
        .expect("failed to watch content folder!");
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
