use chrono::{DateTime, Utc};
use pgvector::Vector;

/// Pagination parameters.
#[derive(Clone, Copy, Debug)]
pub struct PageParams {
    pub offset: u32,
    pub limit: u32,
}

/// `ed_api.corpora` value.
#[derive(Clone, Debug)]
pub struct CorpusValue {
    pub message_id: Option<uuid::Uuid>,
    pub user_id: uuid::Uuid,
    pub body: Vec<u8>,
    pub audience: String,
}

/// `ed_api.fingerprints` value.
#[derive(Clone, Debug)]
pub struct FingerprintValue {
    pub fingerprint_id: Option<uuid::Uuid>,
    pub user_id: uuid::Uuid,
    pub formality_score: f32,
    pub avg_sentence_length: f32,
    pub sentence_length_variance: f32,
    pub exclamation_ratio: f32,
    pub ellipsis_ratio: f32,
    pub emoji_frequency: f32,
    pub contraction_ratio: f32,
    pub hedging_ratio: f32,
    pub common_openers: Vec<String>,
    pub common_closers: Vec<String>,
    pub message_count: i32,
    pub created_at: DateTime<Utc>,
}

/// `ed_api.embeddings` value.
#[derive(Clone, Debug)]
pub struct EmbeddingValue {
    pub embedding_id: Option<uuid::Uuid>,
    pub message_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub vector: Vector,
}

/// A columnar version of `Vec<EmbeddingValue>`, since it's more efficient to
/// insert this way.  Also the number of prepared statements is the number of
/// columns.  With many INSERT INTO ... VALUES you get `n * m`, n the number of
/// values being inserted, m the number of columns.
///
/// Postgres imposes a maximum so this is a real concern.
#[derive(Clone, Debug, Default)]
pub struct EmbeddingValueColumnar<'a> {
    pub message_ids: Vec<uuid::Uuid>,
    pub user_ids: Vec<uuid::Uuid>,
    pub vectors: Vec<&'a Vector>,
}

impl<'a> EmbeddingValueColumnar<'a> {
    pub(crate) fn from_slice(
        vs: &'a [EmbeddingValue],
    ) -> EmbeddingValueColumnar<'a> {
        vs.iter().fold(Self::default(), |mut acc, v| {
            acc.message_ids.push(v.message_id);
            acc.user_ids.push(v.user_id);
            acc.vectors.push(&v.vector);
            acc
        })
    }
}

/// `ed_api.simulations` value.
#[derive(Clone, Debug)]
pub struct SimulationValue {
    pub simulation_id: Option<uuid::Uuid>,
    pub user_id: uuid::Uuid,
    pub prompt: String,
    pub audience: String,
    pub nudge: Option<String>,
    pub generated_text: String,
    pub confidence_overall: f32,
    pub confidence_dimensions: serde_json::Value,
    pub retrieved_examples: serde_json::Value,
    pub fingerprint_snapshot: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

/// Value returned by the `get_similarity` query.
#[derive(Clone, Debug)]
pub struct SimilarityValue {
    pub message_id: uuid::Uuid,
    pub body: Vec<u8>,
    pub audience: String,
    pub similarity: f32,
}

/// Value returned by the `get_corpus` query.
#[derive(Clone, Debug)]
pub struct MessageValue {
    pub message_id: uuid::Uuid,
    pub body: Vec<u8>,
    pub audience: String,
    pub created_at: DateTime<Utc>,
}
