use crate::config::Config;
use crate::http::{HttpServer, HttpServerConfig};
use crate::sqlite::Sqlite;

mod config;
mod error;
mod helper_types;
mod http;
mod models;
mod schema;

mod sqlite;
#[cfg(test)]
mod test;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;

    let sqlite = Sqlite::new(config.database_url())?;

    let server_config = HttpServerConfig::new(config.server_port());
    let http_server = HttpServer::new(sqlite, server_config).await?;
    http_server.run().await
}
