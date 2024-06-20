use diesel::{Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use rand::Rng;

use crate::{establish_pool, run_migrations, Pool, MIGRATIONS};

const TEST_MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/test/");

pub struct TestContext {
    database_path: String,
}

impl TestContext {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let number: u32 = rng.gen();

        let database_path = format!(".__diesel_test_{number}.db");

        let mut connection =
            SqliteConnection::establish(&database_path).expect("Could not connect to database");
        run_migrations(&mut connection, MIGRATIONS);
        run_migrations(&mut connection, TEST_MIGRATIONS);

        Self {
            database_path: String::from(database_path),
        }
    }

    pub async fn pool(&self) -> Pool {
        establish_pool(&self.database_path)
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        std::fs::remove_file(&self.database_path).expect("Unable to delete test database");
    }
}
