use tern::error::TernResult;
use tern::executor::SqlxPgExecutor;
use tern::{ContextOptions, MigrationContext, MigrationSource};

/// Migration runner.
#[derive(MigrationContext, MigrationSource)]
#[tern(source = "src/migrations", table = "_ed_api_history")]
pub struct EdApiMigrations {
    #[tern(executor_via)]
    pub exec: SqlxPgExecutor,
}

impl EdApiMigrations {
    /// New from the connection string.
    pub async fn new(db_url: &str) -> TernResult<Self> {
        let exec = SqlxPgExecutor::new(db_url).await?;
        Ok(Self { exec })
    }
}

/// Boilerplate to make a CLI work.
pub struct EdMigrationOptions;

impl ContextOptions for EdMigrationOptions {
    type Ctx = EdApiMigrations;

    async fn connect(&self, db_url: &str) -> TernResult<Self::Ctx> {
        EdApiMigrations::new(db_url).await
    }
}
