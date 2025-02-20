use crate::sqlite::Sqlite;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use rand::Rng;

const TEST_MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/test/");

#[derive(Debug)]
pub struct TestContext {
    database_path: String,
    database: Sqlite,
}

impl TestContext {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let number: u32 = rng.random();

        let database_path = format!(".__diesel_test_{number}.db");
        let database = Sqlite::new(&database_path).expect("Failed to create SQLite database");
        database
            .connection()
            .unwrap()
            .run_pending_migrations(TEST_MIGRATIONS)
            .expect("Failed to run migrations on database");

        Self {
            database_path,
            database,
        }
    }

    pub const fn database(&self) -> &Sqlite {
        &self.database
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        std::fs::remove_file(&self.database_path).expect("Failed to delete test database");
    }
}
