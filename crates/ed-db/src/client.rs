use anyhow::Context as _;
use futures::future::{BoxFuture, FutureExt as _, TryFutureExt as _};
use futures::stream::TryStreamExt as _;
use log::LevelFilter;
use pgvector::Vector;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions as _, PgPool};
use std::ops::{Deref, DerefMut};
use std::time::Duration;
use url::Url;

use crate::query::{ReadEdApiSchema, WriteEdApiSchema};
use crate::types::*;

/// Configures a [`EdDbClient`].
#[derive(Clone, Debug)]
pub struct EdDbConfig {
    name: String,
    max_conn: u32,
    acquire_timeout: Duration,
    log_level: LevelFilter,
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

    pub async fn try_new_client<T: AsRef<str>>(
        self,
        db_url: T,
    ) -> anyhow::Result<EdDbClient> {
        EdDbClient::new(self, db_url).await
    }
}

/// Client of ed-db.
#[derive(Clone, Debug)]
pub struct EdDbClient(PgPool);

impl EdDbClient {
    pub async fn new<T: AsRef<str>>(
        config: EdDbConfig,
        db_url: T,
    ) -> anyhow::Result<Self> {
        let url = Url::parse(db_url.as_ref())?;
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

impl ReadEdApiSchema for EdDbClient {
    fn get_corpus<'v, 'a: 'v>(
        &'a self,
        user_id: uuid::Uuid,
        audience: &'v str,
        page: PageParams,
    ) -> BoxFuture<'v, anyhow::Result<Vec<MessageValue>>> {
        sqlx::query_file_as!(
            MessageValue,
            "src/sql/get_corpus.sql",
            user_id,
            audience,
            page.limit as i64,
            page.offset as i64,
        )
        .fetch_all(&self.0)
        .map_err(anyhow::Error::from)
        .boxed()
    }

    fn get_fingerprint(
        &self,
        user_id: uuid::Uuid,
    ) -> BoxFuture<'_, anyhow::Result<Option<FingerprintValue>>> {
        sqlx::query_file_as!(
            FingerprintValue,
            "src/sql/get_fingerprint.sql",
            user_id,
        )
        .fetch_optional(&self.0)
        .map_err(anyhow::Error::from)
        .boxed()
    }

    fn get_simulation(
        &self,
        user_id: uuid::Uuid,
        simulation_id: uuid::Uuid,
    ) -> BoxFuture<'_, anyhow::Result<Option<SimulationValue>>> {
        sqlx::query_file_as!(
            SimulationValue,
            "src/sql/get_simulation.sql",
            user_id,
            simulation_id,
        )
        .fetch_optional(&self.0)
        .map_err(anyhow::Error::from)
        .boxed()
    }

    fn get_simulations<'v, 'a: 'v>(
        &'a self,
        user_id: uuid::Uuid,
        audience: &'v str,
        page: PageParams,
    ) -> BoxFuture<'v, anyhow::Result<Vec<SimulationValue>>> {
        sqlx::query_file_as!(
            SimulationValue,
            "src/sql/get_simulations.sql",
            user_id,
            audience,
            page.limit as i64,
            page.offset as i64,
        )
        .fetch_all(&self.0)
        .map_err(anyhow::Error::from)
        .boxed()
    }

    fn get_similarity<'v, 'a: 'v>(
        &'a self,
        user_id: uuid::Uuid,
        vector: &'v Vector,
        limit: i64,
    ) -> BoxFuture<'v, anyhow::Result<Vec<SimilarityValue>>> {
        sqlx::query_file_as!(
            SimilarityValue,
            "src/sql/get_similarity.sql",
            vector as &Vector,
            user_id,
            limit,
        )
        .fetch_all(&self.0)
        .map_err(anyhow::Error::from)
        .boxed()
    }
}

impl WriteEdApiSchema for EdDbClient {
    fn insert_corpus<'v, 'a: 'v>(
        &'a self,
        value: &'v CorpusValue,
    ) -> BoxFuture<'v, anyhow::Result<uuid::Uuid>> {
        sqlx::query_file_scalar!(
            "src/sql/insert_corpus.sql",
            value.user_id,
            value.body,
            &value.audience,
        )
        .fetch_one(&self.0)
        .map_err(anyhow::Error::from)
        .boxed()
    }

    fn delete_corpus<'v, 'a: 'v>(
        &'a self,
        user_id: uuid::Uuid,
    ) -> BoxFuture<'v, anyhow::Result<()>> {
        sqlx::query!("DELETE FROM ed_api.corpora WHERE user_id = $1", user_id)
            .execute(&self.0)
            .map_ok(|_| ())
            .map_err(anyhow::Error::from)
            .boxed()
    }

    fn insert_fingerprint<'v, 'a: 'v>(
        &'a self,
        value: &'v FingerprintValue,
    ) -> BoxFuture<'v, anyhow::Result<uuid::Uuid>> {
        sqlx::query_file_scalar!(
            "src/sql/insert_fingerprint.sql",
            value.user_id,
            value.formality_score,
            value.avg_sentence_length,
            value.sentence_length_variance,
            value.exclamation_ratio,
            value.ellipsis_ratio,
            value.emoji_frequency,
            value.contraction_ratio,
            value.hedging_ratio,
            value.common_openers.as_slice(),
            value.common_closers.as_slice(),
            value.message_count,
        )
        .fetch_one(&self.0)
        .map_err(anyhow::Error::from)
        .boxed()
    }

    fn insert_embeddings<'v, 'a: 'v>(
        &'a self,
        value: &'v Vec<EmbeddingValue>,
    ) -> BoxFuture<'v, anyhow::Result<Vec<uuid::Uuid>>> {
        Box::pin(async move {
            let columnar = EmbeddingValueColumnar::from_vec(value);
            let res = sqlx::query_file_scalar!(
                "src/sql/insert_embeddings.sql",
                &columnar.message_ids.as_slice(),
                &columnar.user_ids.as_slice(),
                &columnar.vectors.as_slice() as _,
            )
            .fetch(&self.0)
            .try_collect::<Vec<_>>()
            .await?;
            Ok(res)
        })
    }

    fn insert_simulation<'v, 'a: 'v>(
        &'a self,
        value: &'v SimulationValue,
    ) -> BoxFuture<'v, anyhow::Result<uuid::Uuid>> {
        sqlx::query_file_scalar!(
            "src/sql/insert_simulation.sql",
            value.user_id,
            value.prompt,
            value.audience,
            value.nudge.as_deref(),
            value.generated_text,
            value.confidence_overall,
            value.confidence_dimensions,
            value.retrieved_examples,
            value.fingerprint_snapshot,
        )
        .fetch_one(&self.0)
        .map_err(anyhow::Error::from)
        .boxed()
    }

    fn delete_simulation<'v, 'a: 'v>(
        &'a self,
        user_id: uuid::Uuid,
        simulation_id: uuid::Uuid,
    ) -> BoxFuture<'v, anyhow::Result<bool>> {
        sqlx::query_file!(
            "src/sql/delete_simulation.sql",
            user_id,
            simulation_id,
        )
        .execute(&self.0)
        .map_ok(|r| r.rows_affected() > 0)
        .map_err(anyhow::Error::from)
        .boxed()
    }
}
