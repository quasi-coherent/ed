use anyhow::Result;
use chrono::{DateTime, Utc};
use futures::future::BoxFuture;
use pgvector::Vector;

use crate::types::*;

/// Scope the session to a user.
pub trait UserScoped {
    /// Set the user_id config value.
    fn set_user_id(&self, user_id: uuid::Uuid) -> BoxFuture<'_, Result<()>>;
}

/// Interacting with the user table.
pub trait ReadWriteEdUsers: Send + Sync {
    /// Insert a new user.
    fn create_user<'v, 'a: 'v>(
        &'a self,
        account_id: &'v str,
        username: &'v str,
        email: &'v str,
    ) -> BoxFuture<'v, Result<uuid::Uuid>>;

    /// Get a user by the provider's ID for them.
    fn get_user<'v, 'a: 'v>(
        &'a self,
        account_id: &'v str,
    ) -> BoxFuture<'v, Result<Option<UserValue>>>;

    /// Delete a user.
    fn delete_user(&self, user_id: uuid::Uuid) -> BoxFuture<'_, Result<bool>>;

    /// Save a new user session returning the session ID.
    fn new_user_session(
        &self,
        user_id: uuid::Uuid,
        created_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
    ) -> BoxFuture<'_, Result<uuid::Uuid>>;

    /// Get a session.
    fn get_user_session(
        &self,
        session_id: uuid::Uuid,
    ) -> BoxFuture<'_, Result<Option<UserSessionValue>>>;

    /// Delete a user session.
    fn delete_user_session(
        &self,
        session_id: uuid::Uuid,
    ) -> BoxFuture<'_, Result<bool>>;
}

/// Interface for read access on the API tables.
pub trait ReadEdApiSchema: Send + Sync {
    /// `ed_api.corpora` select.
    fn get_corpus<'v, 'a: 'v>(
        &'a self,
        user_id: uuid::Uuid,
        audience: &'v str,
        page: PageParams,
    ) -> BoxFuture<'v, Result<Vec<MessageValue>>>;

    /// `ed_api.fingerprints` select.
    fn get_fingerprint(
        &self,
        user_id: uuid::Uuid,
    ) -> BoxFuture<'_, Result<Option<FingerprintValue>>>;

    /// `ed_api.simulations` select ID.
    fn get_simulation(
        &self,
        user_id: uuid::Uuid,
        simulation_id: uuid::Uuid,
    ) -> BoxFuture<'_, Result<Option<SimulationValue>>>;

    /// `ed_api.simulations` select.
    fn get_simulations<'v, 'a: 'v>(
        &'a self,
        user_id: uuid::Uuid,
        audience: &'v str,
        page: PageParams,
    ) -> BoxFuture<'v, Result<Vec<SimulationValue>>>;

    /// Top-N nearest corpus messages by cosine similarity.
    fn get_similarity<'v, 'a: 'v>(
        &'a self,
        user_id: uuid::Uuid,
        vector: &'v Vector,
        limit: i64,
    ) -> BoxFuture<'v, Result<Vec<SimilarityValue>>>;
}

/// Interface for write access on the API tables.
pub trait WriteEdApiSchema: Send + Sync {
    /// `ed_api.corpora` insert.
    fn insert_corpus<'v, 'a: 'v>(
        &'a self,
        value: &'v CorpusValue,
    ) -> BoxFuture<'v, Result<uuid::Uuid>>;

    /// Delete from `ed_api.corpora`.
    fn delete_corpus(&self, user_id: uuid::Uuid)
    -> BoxFuture<'_, Result<bool>>;

    /// `ed_api.fingerprints` insert.
    fn insert_fingerprint<'v, 'a: 'v>(
        &'a self,
        value: &'v FingerprintValue,
    ) -> BoxFuture<'v, Result<uuid::Uuid>>;

    /// `ed_api.embeddings` insert.
    fn insert_embeddings<'v, 'a: 'v>(
        &'a self,
        value: &'v [EmbeddingValue],
    ) -> BoxFuture<'v, Result<Vec<uuid::Uuid>>>;

    /// `ed_api.simulations` insert.
    fn insert_simulation<'v, 'a: 'v>(
        &'a self,
        value: &'v SimulationValue,
    ) -> BoxFuture<'v, Result<uuid::Uuid>>;

    /// Delete one simulation. Returns whether a row was deleted.
    fn delete_simulation(
        &self,
        user_id: uuid::Uuid,
        simulation_id: uuid::Uuid,
    ) -> BoxFuture<'_, Result<bool>>;
}
