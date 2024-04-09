use axum::{routing::get, Router};
use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use dotenvy::dotenv;
use std::env;

use crate::handlers::{create_tag, delete_tag, get_tag, list_tags, root};

mod handlers;
mod helper_types;
mod models;
mod schema;

type Pool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::SqliteConnection>>;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = establish_connection();
    let app = build_router(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn establish_connection() -> Pool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(&database_url);
    Pool::builder()
        .build(manager)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}

fn build_router(pool: Pool) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/tags", get(list_tags).post(create_tag))
        .route("/tags/:id", get(get_tag).delete(delete_tag))
        .with_state(pool)
}
