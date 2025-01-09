use crate::config::Config;
use crate::http::{HttpServer, HttpServerConfig};
use diesel::{r2d2::ConnectionManager, Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

mod config;
mod error;
mod helper_types;
mod http;
mod models;
mod schema;

#[cfg(test)]
mod test;

type Pool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::SqliteConnection>>;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/sqlite/");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;

    initialize_database(config.database_url());
    let pool = establish_pool(config.database_url());

    let server_config = HttpServerConfig::new(config.server_port());
    let http_server = HttpServer::new(pool, server_config).await?;
    http_server.run().await
}

fn initialize_database(database_path: &str) {
    let mut connection =
        SqliteConnection::establish(database_path).expect("Could not connect to database");
    run_migrations(&mut connection, MIGRATIONS);
}

fn run_migrations(
    connection: &mut impl MigrationHarness<diesel::sqlite::Sqlite>,
    embedded_migrations: EmbeddedMigrations,
) {
    connection
        .run_pending_migrations(embedded_migrations)
        .expect("Could not run database migrations");
}

fn establish_pool(database_path: &str) -> Pool {
    let manager = ConnectionManager::<SqliteConnection>::new(database_path);
    Pool::builder()
        .build(manager)
        .unwrap_or_else(|_| panic!("Could not connect to {database_path}"))
}
