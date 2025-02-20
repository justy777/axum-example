use anyhow::Context;
use diesel::SqliteConnection;
use diesel::r2d2::ConnectionManager;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

type Pool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::SqliteConnection>>;
type Connection = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>>;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/sqlite/");

#[derive(Debug, Clone)]
pub struct Sqlite {
    pool: Pool,
}

impl Sqlite {
    pub fn new(path: &str) -> anyhow::Result<Self> {
        let manager = ConnectionManager::<SqliteConnection>::new(path);
        let pool = Pool::builder()
            .build(manager)
            .with_context(|| format!("Failed to open database at {path}"))?;

        pool.get()?
            .run_pending_migrations(MIGRATIONS)
            .map_err(|err| anyhow::anyhow!(err))
            .context("Failed to run migrations on database")?;

        Ok(Self { pool })
    }

    pub fn connection(&self) -> anyhow::Result<Connection> {
        self.pool.get().context("Failed to get database connection")
    }
}
