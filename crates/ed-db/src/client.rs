use anyhow::Context as _;
use chrono::{DateTime, Utc};
use futures::future::{BoxFuture, FutureExt as _, TryFutureExt as _};
use log::LevelFilter;
use pgvector::Vector;
use secrecy::{ExposeSecret as _, SecretString};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions as _, PgPool};
use std::ops::{Deref, DerefMut};
use std::time::Duration;
use url::Url;

use crate::query::{
    ReadEdApiSchema, ReadWriteEdUsers, UserScoped, WriteEdApiSchema,
};
use crate::sql;
use crate::types::*;

/// Db config.
#[derive(Clone, Debug)]
pub struct EdDbConfig {
    name: String,
    max_conn: u32,
    acquire_timeout: Duration,
    log_level: LevelFilter,
}

impl Default for EdDbConfig {
    fn default() -> Self {
        Self::new("ed-db")
    }
}

impl EdDbConfig {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self {
            name: name.into(),
            max_conn: 2,
            acquire_timeout: Duration::from_millis(500),
            log_level: LevelFilter::Info,
        }
    }

    pub fn set_max_conn(self, max_conn: u32) -> Self {
        Self { max_conn, ..self }
    }

    pub fn set_acquire_timeout_millis(self, timeout_millis: u64) -> Self {
        Self { acquire_timeout: Duration::from_millis(timeout_millis), ..self }
    }

    pub fn with_debug_logging(self) -> Self {
        Self { log_level: LevelFilter::Debug, ..self }
    }

    pub async fn try_new_client(
        self,
        db_url: &SecretString,
    ) -> anyhow::Result<EdDbClient> {
        EdDbClient::new(self, db_url).await
    }
}

/// Client of ed-db.
#[derive(Clone, Debug)]
pub struct EdDbClient(PgPool);

impl EdDbClient {
    pub async fn new(
        config: EdDbConfig,
        db_url: &SecretString,
    ) -> anyhow::Result<Self> {
        let url = Url::parse(db_url.expose_secret())?;
        let conn_opt = PgConnectOptions::from_url(&url)?
            .application_name(&config.name)
            .log_statements(config.log_level);
        PgPoolOptions::new()
            .acquire_timeout(config.acquire_timeout)
            .max_connections(config.max_conn)
            .connect_with(conn_opt)
            .await
            .context("establishing db conn")
            .map(Self)
    }

    pub fn pg_sql<'a>(&'a self) -> sql::PgSql<&'a PgPool> {
        sql::PgSql(&self.0)
    }
}

impl Deref for EdDbClient {
    type Target = PgPool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EdDbClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl UserScoped for EdDbClient {
    fn set_user_id(
        &self,
        user_id: uuid::Uuid,
    ) -> BoxFuture<'_, anyhow::Result<()>> {
        sqlx::query_scalar!(
            r#"SELECT set_config('ed_api.user_id', $1, false);"#,
            user_id.to_string()
        )
        .fetch_one(&self.0)
        .map_ok_or_else(
            |e| Err(anyhow::Error::from(e)),
            move |res| {
                if let Some(val) = res.as_ref()
                    && let Ok(id) = uuid::Uuid::parse_str(val)
                    && id == user_id
                {
                    return Ok(());
                }
                Err(anyhow::anyhow!("set user_id session config"))
            },
        )
        .boxed()
    }
}

impl ReadWriteEdUsers for EdDbClient {
    fn create_user<'v, 'a: 'v>(
        &'a self,
        account_id: &'v str,
        username: &'v str,
        email: &'v str,
    ) -> BoxFuture<'v, anyhow::Result<uuid::Uuid>> {
        self.pg_sql().create_user(account_id, username, email).boxed()
    }

    fn get_user<'v, 'a: 'v>(
        &'a self,
        account_id: &'v str,
    ) -> BoxFuture<'v, anyhow::Result<Option<UserValue>>> {
        self.pg_sql().get_user(account_id).boxed()
    }

    fn delete_user(
        &self,
        user_id: uuid::Uuid,
    ) -> BoxFuture<'_, anyhow::Result<bool>> {
        self.pg_sql().delete_user(user_id).boxed()
    }

    fn new_user_session(
        &self,
        user_id: uuid::Uuid,
        created_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
    ) -> BoxFuture<'_, anyhow::Result<uuid::Uuid>> {
        self.pg_sql().new_user_session(user_id, created_at, expires_at).boxed()
    }

    fn get_user_session(
        &self,
        session_id: uuid::Uuid,
    ) -> BoxFuture<'_, anyhow::Result<Option<UserSessionValue>>> {
        self.pg_sql().get_user_session(session_id).boxed()
    }

    fn delete_user_session(
        &self,
        session_id: uuid::Uuid,
    ) -> BoxFuture<'_, anyhow::Result<bool>> {
        self.pg_sql().delete_user_session(session_id).boxed()
    }
}

impl ReadEdApiSchema for EdDbClient {
    fn get_corpus<'v, 'a: 'v>(
        &'a self,
        user_id: uuid::Uuid,
        audience: &'v str,
        page: PageParams,
    ) -> BoxFuture<'v, anyhow::Result<Vec<MessageValue>>> {
        self.pg_sql().get_corpus(user_id, audience, page).boxed()
    }

    fn get_fingerprint(
        &self,
        user_id: uuid::Uuid,
    ) -> BoxFuture<'_, anyhow::Result<Option<FingerprintValue>>> {
        self.pg_sql().get_fingerprint(user_id).boxed()
    }

    fn get_simulation(
        &self,
        user_id: uuid::Uuid,
        simulation_id: uuid::Uuid,
    ) -> BoxFuture<'_, anyhow::Result<Option<SimulationValue>>> {
        self.pg_sql().get_simulation(user_id, simulation_id).boxed()
    }

    fn get_simulations<'v, 'a: 'v>(
        &'a self,
        user_id: uuid::Uuid,
        audience: &'v str,
        page: PageParams,
    ) -> BoxFuture<'v, anyhow::Result<Vec<SimulationValue>>> {
        self.pg_sql().get_simulations(user_id, audience, page).boxed()
    }

    fn get_similarity<'v, 'a: 'v>(
        &'a self,
        user_id: uuid::Uuid,
        vector: &'v Vector,
        limit: i64,
    ) -> BoxFuture<'v, anyhow::Result<Vec<SimilarityValue>>> {
        self.pg_sql().get_similarity(user_id, vector, limit).boxed()
    }
}

impl WriteEdApiSchema for EdDbClient {
    fn insert_corpus<'v, 'a: 'v>(
        &'a self,
        value: &'v CorpusValue,
    ) -> BoxFuture<'v, anyhow::Result<uuid::Uuid>> {
        self.pg_sql().insert_corpus(value).boxed()
    }

    fn delete_corpus(
        &self,
        user_id: uuid::Uuid,
    ) -> BoxFuture<'_, anyhow::Result<bool>> {
        self.pg_sql().delete_corpus(user_id).boxed()
    }

    fn insert_fingerprint<'v, 'a: 'v>(
        &'a self,
        value: &'v FingerprintValue,
    ) -> BoxFuture<'v, anyhow::Result<uuid::Uuid>> {
        self.pg_sql().insert_fingerprint(value).boxed()
    }

    fn insert_embeddings<'v, 'a: 'v>(
        &'a self,
        value: &'v [EmbeddingValue],
    ) -> BoxFuture<'v, anyhow::Result<Vec<uuid::Uuid>>> {
        self.pg_sql().insert_embeddings(value).boxed()
    }

    fn insert_simulation<'v, 'a: 'v>(
        &'a self,
        value: &'v SimulationValue,
    ) -> BoxFuture<'v, anyhow::Result<uuid::Uuid>> {
        self.pg_sql().insert_simulation(value).boxed()
    }

    fn delete_simulation(
        &self,
        user_id: uuid::Uuid,
        simulation_id: uuid::Uuid,
    ) -> BoxFuture<'_, anyhow::Result<bool>> {
        self.pg_sql().delete_simulation(user_id, simulation_id).boxed()
    }
}
