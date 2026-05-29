use anyhow::Result;
use futures::future::BoxFuture;
use pgvector::Vector;

use crate::types::*;

/// Interface for read access on the API tables.
pub trait ReadEdApiSchema {
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
pub trait WriteEdApiSchema {
    /// `ed_api.corpora` insert.
    fn insert_corpus<'v, 'a: 'v>(
        &'a self,
        value: &'v CorpusValue,
    ) -> BoxFuture<'v, Result<uuid::Uuid>>;

    /// Delete from `ed_api.corpora`.
    fn delete_corpus<'v, 'a: 'v>(
        &'a self,
        user_id: uuid::Uuid,
    ) -> BoxFuture<'v, Result<()>>;

    /// `ed_api.fingerprints` insert.
    fn insert_fingerprint<'v, 'a: 'v>(
        &'a self,
        value: &'v FingerprintValue,
    ) -> BoxFuture<'v, Result<uuid::Uuid>>;

    /// `ed_api.embeddings` insert.
    fn insert_embeddings<'v, 'a: 'v>(
        &'a self,
        value: &'v Vec<EmbeddingValue>,
    ) -> BoxFuture<'v, Result<Vec<uuid::Uuid>>>;

    /// `ed_api.simulations` insert.
    fn insert_simulation<'v, 'a: 'v>(
        &'a self,
        value: &'v SimulationValue,
    ) -> BoxFuture<'v, Result<uuid::Uuid>>;

    /// Delete one simulation. Returns whether a row was deleted.
    fn delete_simulation<'v, 'a: 'v>(
        &'a self,
        user_id: uuid::Uuid,
        simulation_id: uuid::Uuid,
    ) -> BoxFuture<'v, Result<bool>>;
}
