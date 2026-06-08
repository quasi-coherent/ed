use chrono::{DateTime, Utc};
use futures::future::TryFutureExt as _;
use futures::stream::TryStreamExt as _;
use pgvector::Vector;
use sqlx::PgExecutor;

use crate::types::*;

/// Query library.
#[derive(Debug, Clone, Copy, Default)]
pub struct PgSql<E>(pub E);

impl<E> PgSql<E>
where
    for<'a> E: PgExecutor<'a>,
{
    pub async fn create_user(
        self,
        account_id: &str,
        username: &str,
        email: &str,
    ) -> anyhow::Result<uuid::Uuid> {
        sqlx::query_scalar!(
            r#"
INSERT INTO ed_api.users (account_id, username, email)
  VALUES ($1, $2, $3)
  RETURNING (id);
"#,
            account_id,
            username,
            email,
        )
        .fetch_one(self.0)
        .map_err(anyhow::Error::from)
        .await
    }

    pub async fn get_user(
        self,
        account_id: &str,
    ) -> anyhow::Result<Option<UserValue>> {
        sqlx::query_as!(
            UserValue,
            r#"
SELECT
  id AS user_id,
  account_id,
  username,
  email,
  created_at
FROM
  ed_api.users
WHERE
  account_id = $1;
"#,
            account_id,
        )
        .fetch_optional(self.0)
        .map_err(anyhow::Error::from)
        .await
    }

    pub async fn delete_user(
        self,
        user_id: uuid::Uuid,
    ) -> anyhow::Result<bool> {
        sqlx::query!(
            r#"
DELETE FROM ed_api.users
WHERE id = $1;
"#,
            user_id,
        )
        .execute(self.0)
        .map_ok(|r| r.rows_affected() > 0)
        .map_err(anyhow::Error::from)
        .await
    }

    pub async fn new_user_session(
        self,
        user_id: uuid::Uuid,
        created_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
    ) -> anyhow::Result<uuid::Uuid> {
        sqlx::query_scalar!(
            r#"
INSERT INTO ed_api.sessions (user_id, created_at, expires_at)
  VALUES ($1, $2, $3)
  RETURNING (id);
"#,
            user_id,
            created_at,
            expires_at,
        )
        .fetch_one(self.0)
        .map_err(anyhow::Error::from)
        .await
    }

    pub async fn get_user_session(
        self,
        session_id: uuid::Uuid,
    ) -> anyhow::Result<Option<UserSessionValue>> {
        sqlx::query_as!(
            UserSessionValue,
            r#"
SELECT
  id AS session_id,
  user_id,
  created_at,
  expires_at
FROM
  ed_api.sessions
WHERE
  id = $1;
"#,
            session_id,
        )
        .fetch_optional(self.0)
        .map_err(anyhow::Error::from)
        .await
    }

    pub async fn delete_user_session(
        self,
        session_id: uuid::Uuid,
    ) -> anyhow::Result<bool> {
        sqlx::query!("DELETE FROM ed_api.sessions WHERE id = $1", session_id)
            .execute(self.0)
            .map_ok(|r| r.rows_affected() > 0)
            .map_err(anyhow::Error::from)
            .await
    }

    pub async fn get_corpus(
        self,
        user_id: uuid::Uuid,
        audience: &str,
        page: PageParams,
    ) -> anyhow::Result<Vec<MessageValue>> {
        sqlx::query_file_as!(
            MessageValue,
            "src/sql/get_corpus.sql",
            user_id,
            audience,
            page.limit as i64,
            page.offset as i64,
        )
        .fetch_all(self.0)
        .map_err(anyhow::Error::from)
        .await
    }

    pub async fn get_fingerprint(
        self,
        user_id: uuid::Uuid,
    ) -> anyhow::Result<Option<FingerprintValue>> {
        sqlx::query_file_as!(
            FingerprintValue,
            "src/sql/get_fingerprint.sql",
            user_id,
        )
        .fetch_optional(self.0)
        .map_err(anyhow::Error::from)
        .await
    }

    pub async fn get_simulation(
        self,
        user_id: uuid::Uuid,
        simulation_id: uuid::Uuid,
    ) -> anyhow::Result<Option<SimulationValue>> {
        sqlx::query_file_as!(
            SimulationValue,
            "src/sql/get_simulation.sql",
            user_id,
            simulation_id,
        )
        .fetch_optional(self.0)
        .map_err(anyhow::Error::from)
        .await
    }

    pub async fn get_simulations(
        self,
        user_id: uuid::Uuid,
        audience: &str,
        page: PageParams,
    ) -> anyhow::Result<Vec<SimulationValue>> {
        sqlx::query_file_as!(
            SimulationValue,
            "src/sql/get_simulations.sql",
            user_id,
            audience,
            page.limit as i64,
            page.offset as i64,
        )
        .fetch_all(self.0)
        .map_err(anyhow::Error::from)
        .await
    }

    pub async fn get_similarity(
        self,
        user_id: uuid::Uuid,
        vector: &Vector,
        limit: i64,
    ) -> anyhow::Result<Vec<SimilarityValue>> {
        sqlx::query_file_as!(
            SimilarityValue,
            "src/sql/get_similarity.sql",
            vector as &Vector,
            user_id,
            limit,
        )
        .fetch_all(self.0)
        .map_err(anyhow::Error::from)
        .await
    }

    pub async fn insert_corpus(
        self,
        value: &CorpusValue,
    ) -> anyhow::Result<uuid::Uuid> {
        sqlx::query_scalar!(
            r#"
INSERT INTO ed_api.corpora (user_id, body, audience)
  VALUES ($1, $2, $3)
  RETURNING id;
"#,
            value.user_id,
            value.body,
            &value.audience,
        )
        .fetch_one(self.0)
        .map_err(anyhow::Error::from)
        .await
    }

    pub async fn delete_corpus(
        self,
        user_id: uuid::Uuid,
    ) -> anyhow::Result<bool> {
        sqlx::query!("DELETE FROM ed_api.corpora WHERE user_id = $1", user_id)
            .execute(self.0)
            .map_ok(|r| r.rows_affected() > 0)
            .map_err(anyhow::Error::from)
            .await
    }

    pub async fn insert_fingerprint(
        self,
        value: &FingerprintValue,
    ) -> anyhow::Result<uuid::Uuid> {
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
        .fetch_one(self.0)
        .map_err(anyhow::Error::from)
        .await
    }

    pub async fn insert_embeddings(
        self,
        value: &[EmbeddingValue],
    ) -> anyhow::Result<Vec<uuid::Uuid>> {
        let columnar = EmbeddingValueColumnar::from_slice(value);
        let res = sqlx::query_scalar!(
            r#"
INSERT INTO ed_api.embeddings (message_id, user_id, vector)
  SELECT *
  FROM unnest(
    $1::uuid[],
    $2::uuid[],
    $3::vector[]
  )
  RETURNING id;
"#,
            &columnar.message_ids.as_slice(),
            &columnar.user_ids.as_slice(),
            &columnar.vectors.as_slice() as _,
        )
        .fetch(self.0)
        .try_collect::<Vec<_>>()
        .await?;
        Ok(res)
    }

    pub async fn insert_simulation(
        self,
        value: &SimulationValue,
    ) -> anyhow::Result<uuid::Uuid> {
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
        .fetch_one(self.0)
        .map_err(anyhow::Error::from)
        .await
    }

    pub async fn delete_simulation(
        self,
        user_id: uuid::Uuid,
        simulation_id: uuid::Uuid,
    ) -> anyhow::Result<bool> {
        sqlx::query!(
            r#"
DELETE FROM ed_api.simulations
WHERE
  id = $1
  AND user_id = $2;
"#,
            simulation_id,
            user_id,
        )
        .execute(self.0)
        .map_ok(|r| r.rows_affected() > 0)
        .map_err(anyhow::Error::from)
        .await
    }
}
