mod handler;

use crate::http::handler::{create_tag, delete_tag, get_tag, list_tags};
use crate::sqlite::Sqlite;
use anyhow::Context;
use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;

#[derive(Debug)]
pub struct HttpServerConfig {
    port: String,
}

impl HttpServerConfig {
    pub fn new(port: &str) -> Self {
        Self { port: port.into() }
    }
}

pub struct HttpServer {
    router: Router,
    listener: TcpListener,
}

impl HttpServer {
    pub async fn new(database: Sqlite, config: HttpServerConfig) -> anyhow::Result<Self> {
        let router = Router::new()
            .nest("/api/v1", api_routes())
            .with_state(database);

        let listener = TcpListener::bind(format!("0.0.0.0:{}", config.port))
            .await
            .with_context(|| format!("Failed to bind to port {}", config.port))?;

        Ok(Self { router, listener })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        axum::serve(self.listener, self.router)
            .await
            .context("Received error from running server")?;
        Ok(())
    }
}

pub fn api_routes() -> Router<Sqlite> {
    Router::new()
        .route("/tags", get(list_tags).post(create_tag))
        .route("/tags/:id", get(get_tag).delete(delete_tag))
}
