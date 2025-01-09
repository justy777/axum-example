use crate::config::Config;
use crate::handlers::{create_tag, delete_tag, get_tag, list_tags};
use anyhow::Context;
use axum::{routing::get, Router};
use diesel::{r2d2::ConnectionManager, Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

mod config;
mod error;
mod handlers;
mod helper_types;
mod models;
mod schema;

#[cfg(test)]
mod test;

type Pool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::SqliteConnection>>;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/sqlite/");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;

    initialize_database(&config.database_url());
    let pool = establish_pool(&config.database_url());
    let app = build_router(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .with_context(|| format!("Failed to bind to port {}", config.server_port()))?;
    axum::serve(listener, app)
        .await
        .context("Received error from running server")?;
    Ok(())
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

pub fn build_router(pool: Pool) -> Router {
    Router::new()
        .route("/api/v1/tags", get(list_tags).post(create_tag))
        .route("/api/v1/tags/:id", get(get_tag).delete(delete_tag))
        .with_state(pool)
}
