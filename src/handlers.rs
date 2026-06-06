//! Per-route handler bodies. Each returns `Result<T, AppError>`; the trait
//! impl in `router.rs` maps these into the generated response enums.
use chrono::Utc;
use ed_axum::models::*;
use ed_clients::types::{EmbeddingsRequest, MessagesRequest};
use ed_db::types::*;
use ed_types::fingerprint;
use futures::stream::{self, StreamExt as _, TryStreamExt as _};
use pgvector::Vector;
use std::str::FromStr;
use uuid::Uuid;

use crate::error::AppError;
use crate::prompt::{render_prompt, score_confidence};
use crate::state::AppState;

pub async fn ingest(
    state: &AppState,
    user_id: Uuid,
    upload: &CorpusUpload,
) -> Result<IngestResult, AppError> {
    let messages = fingerprint::parse(upload);
    let message_count = messages.len() as i32;

    let fp = fingerprint::analyze(&messages)
        .map_err(|e| AppError::InvalidInput(e.to_string()))?;

    let st = stream::iter(messages);

    let embeddings = st
        .then(|msg| async move {
            let embedding = state
                .services
                .embed(EmbeddingsRequest::new(&msg.body))
                .await
                .map_err(AppError::Client)?;

            let vector = Vector::from(embedding.to_vec());

            let corpus_value = CorpusValue {
                message_id: None,
                user_id,
                body: msg.body.as_bytes().to_vec(),
                audience: msg.audience.to_string(),
            };

            let message_id = state
                .db
                .insert_corpus(&corpus_value)
                .await
                .map_err(AppError::Db)?;

            Ok::<_, AppError>(EmbeddingValue {
                embedding_id: None,
                message_id,
                user_id,
                vector,
            })
        })
        .try_fold(Vec::new(), |mut acc, val| async move {
            acc.push(val);
            Ok(acc)
        })
        .await?;

    let embedding_count = embeddings.len() as i32;

    state.db.insert_embeddings(&embeddings).await.map_err(AppError::Db)?;

    let fp_value = fingerprint_to_value(user_id, &fp);
    state.db.insert_fingerprint(&fp_value).await.map_err(AppError::Db)?;

    Ok(IngestResult { message_count, embedding_count, fingerprint: fp })
}

pub async fn delete_corpus(
    state: &AppState,
    user_id: Uuid,
) -> Result<(), AppError> {
    if !state.db.delete_corpus(user_id).await.map_err(AppError::Db)? {
        return Err(AppError::NotFound);
    }
    Ok(())
}

pub async fn list_corpus(
    state: &AppState,
    user_id: Uuid,
    audience: Option<Audience>,
    limit: Option<u8>,
    offset: Option<u32>,
) -> Result<MessageList, AppError> {
    let limit = limit.map(|l| l as u32).unwrap_or(50).min(200);
    let offset = offset.unwrap_or(0);
    let audience_str = audience.map(|a| a.to_string()).unwrap_or_default();
    let page = PageParams { limit, offset };
    let rows = state
        .db
        .get_corpus(user_id, &audience_str, page)
        .await
        .map_err(AppError::Db)?;
    let messages: Vec<StoredMessage> = rows
        .into_iter()
        .map(value_to_stored_message)
        .collect::<Result<_, _>>()?;
    let total = messages.len() as i32;
    Ok(MessageList {
        messages,
        total,
        limit: limit as i32,
        offset: offset as i32,
    })
}

pub async fn get_fingerprint(
    state: &AppState,
    user_id: Uuid,
) -> Result<StyleFingerprint, AppError> {
    let row = state
        .db
        .get_fingerprint(user_id)
        .await
        .map_err(AppError::Db)?
        .ok_or(AppError::NotFound)?;
    Ok(value_to_fingerprint(&row))
}

// ---- simulate ---------------------------------------------------------------

pub async fn generate(
    state: &AppState,
    user_id: Uuid,
    req: &GenerateRequest,
) -> Result<GenerateResponse, AppError> {
    let retrieval_count = req.retrieval_count.unwrap_or(4) as i64;

    let prompt_embedding = state
        .services
        .embed(EmbeddingsRequest::new(&req.prompt))
        .await
        .map_err(AppError::Client)?;
    let prompt_vector = Vector::from(prompt_embedding.to_vec());

    let similar = state
        .db
        .get_similarity(user_id, &prompt_vector, retrieval_count)
        .await
        .map_err(AppError::Db)?;

    let retrieved_examples: Vec<RetrievedMessage> = similar
        .into_iter()
        .map(similarity_to_retrieved_message)
        .collect::<Result<_, _>>()?;

    let fingerprint_value = state
        .db
        .get_fingerprint(user_id)
        .await
        .map_err(AppError::Db)?
        .ok_or(AppError::NotFound)?;
    let fingerprint_used = value_to_fingerprint(&fingerprint_value);

    let assembled = render_prompt(
        &fingerprint_used,
        &retrieved_examples,
        &req.prompt,
        req.nudge.as_deref(),
    );

    let response = state
        .services
        .prompt(MessagesRequest::new(&assembled))
        .await
        .map_err(AppError::Client)?;
    let generated_text = response.to_string();

    let generated_msgs = vec![fingerprint::Message {
        body: generated_text.clone(),
        audience: req.audience.unwrap_or(Audience::Unknown),
    }];
    let generated_fp = fingerprint::analyze(&generated_msgs)
        .unwrap_or_else(|_| empty_fingerprint());

    let confidence = score_confidence(&fingerprint_used, &generated_fp);

    let result = GenerateResponse {
        generated_text: generated_text.clone(),
        retrieved_examples: retrieved_examples.clone(),
        fingerprint_used: fingerprint_used.clone(),
        confidence: confidence.clone(),
    };

    let audience_str = req
        .audience
        .map(|a| a.to_string())
        .unwrap_or_else(|| Audience::Unknown.to_string());

    let confidence_dimensions = serde_json::to_value(&confidence.dimensions)
        .map_err(|e| {
            AppError::Db(anyhow::anyhow!("serialize dimensions: {e}"))
        })?;
    let retrieved_examples_json = serde_json::to_value(&retrieved_examples)
        .map_err(|e| {
            AppError::Db(anyhow::anyhow!("serialize retrieved: {e}"))
        })?;
    let fingerprint_snapshot = serde_json::to_value(&fingerprint_used)
        .map_err(|e| {
            AppError::Db(anyhow::anyhow!("serialize fingerprint: {e}"))
        })?;

    let value = SimulationValue {
        simulation_id: None,
        user_id,
        prompt: req.prompt.clone(),
        audience: audience_str,
        nudge: req.nudge.clone(),
        generated_text,
        confidence_overall: confidence.overall,
        confidence_dimensions,
        retrieved_examples: retrieved_examples_json,
        fingerprint_snapshot,
        created_at: Utc::now(),
    };
    state.db.insert_simulation(&value).await.map_err(AppError::Db)?;

    Ok(result)
}

pub async fn list_simulations(
    state: &AppState,
    user_id: Uuid,
    limit: Option<u8>,
    offset: Option<u32>,
) -> Result<HistoryList, AppError> {
    let limit = limit.map(|l| l as u32).unwrap_or(20).min(100);
    let offset = offset.unwrap_or(0);
    let page = PageParams { limit, offset };
    let rows = state
        .db
        .get_simulations(user_id, "", page)
        .await
        .map_err(AppError::Db)?;
    let entries: Vec<SimulationEntry> = rows
        .into_iter()
        .map(value_to_simulation_entry)
        .collect::<Result<_, _>>()?;
    let total = entries.len() as i32;
    Ok(HistoryList {
        entries,
        total,
        limit: limit as i32,
        offset: offset as i32,
    })
}

pub async fn get_simulation(
    state: &AppState,
    user_id: Uuid,
    simulation_id: Uuid,
) -> Result<SimulationEntry, AppError> {
    let row = state
        .db
        .get_simulation(user_id, simulation_id)
        .await
        .map_err(AppError::Db)?
        .ok_or(AppError::NotFound)?;
    value_to_simulation_entry(row)
}

pub async fn delete_simulation(
    state: &AppState,
    user_id: Uuid,
    simulation_id: Uuid,
) -> Result<(), AppError> {
    let deleted = state
        .db
        .delete_simulation(user_id, simulation_id)
        .await
        .map_err(AppError::Db)?;
    if deleted { Ok(()) } else { Err(AppError::NotFound) }
}

// ---- conversions ------------------------------------------------------------

fn fingerprint_to_value(
    user_id: Uuid,
    fp: &StyleFingerprint,
) -> FingerprintValue {
    FingerprintValue {
        fingerprint_id: None,
        user_id,
        formality_score: fp.formality_score,
        avg_sentence_length: fp.avg_sentence_length,
        sentence_length_variance: fp.sentence_length_variance,
        exclamation_ratio: fp.exclamation_ratio,
        ellipsis_ratio: fp.ellipsis_ratio,
        emoji_frequency: fp.emoji_frequency,
        contraction_ratio: fp.contraction_ratio,
        hedging_ratio: fp.hedging_ratio,
        common_openers: fp.common_openers.clone(),
        common_closers: fp.common_closers.clone(),
        message_count: fp.message_count,
        created_at: Utc::now(),
    }
}

fn value_to_fingerprint(v: &FingerprintValue) -> StyleFingerprint {
    StyleFingerprint {
        formality_score: v.formality_score,
        avg_sentence_length: v.avg_sentence_length,
        sentence_length_variance: v.sentence_length_variance,
        exclamation_ratio: v.exclamation_ratio,
        ellipsis_ratio: v.ellipsis_ratio,
        emoji_frequency: v.emoji_frequency,
        contraction_ratio: v.contraction_ratio,
        hedging_ratio: v.hedging_ratio,
        common_openers: v.common_openers.clone(),
        common_closers: v.common_closers.clone(),
        message_count: v.message_count,
    }
}

fn value_to_stored_message(v: MessageValue) -> Result<StoredMessage, AppError> {
    let text = std::str::from_utf8(&v.body)?.to_string();
    Ok(StoredMessage {
        message_id: v.message_id,
        text,
        audience: parse_audience(&v.audience),
    })
}

fn similarity_to_retrieved_message(
    v: SimilarityValue,
) -> Result<RetrievedMessage, AppError> {
    let text = std::str::from_utf8(&v.body)?.to_string();
    Ok(RetrievedMessage {
        text,
        audience: parse_audience(&v.audience),
        similarity_score: v.similarity.clamp(0.0, 1.0),
    })
}

fn value_to_simulation_entry(
    v: SimulationValue,
) -> Result<SimulationEntry, AppError> {
    let retrieved_examples: Vec<RetrievedMessage> =
        serde_json::from_value(v.retrieved_examples).map_err(|e| {
            AppError::Db(anyhow::anyhow!("retrieved_examples: {e}"))
        })?;
    let fingerprint_used: StyleFingerprint =
        serde_json::from_value(v.fingerprint_snapshot).map_err(|e| {
            AppError::Db(anyhow::anyhow!("fingerprint_snapshot: {e}"))
        })?;
    let dimensions: Vec<DimensionScore> =
        serde_json::from_value(v.confidence_dimensions).map_err(|e| {
            AppError::Db(anyhow::anyhow!("confidence_dimensions: {e}"))
        })?;
    let confidence =
        ConfidenceScore { overall: v.confidence_overall, dimensions };

    let result = GenerateResponse {
        generated_text: v.generated_text,
        retrieved_examples,
        fingerprint_used,
        confidence,
    };

    Ok(SimulationEntry {
        simulation_id: v.simulation_id.unwrap_or_else(Uuid::nil),
        prompt: v.prompt,
        audience: parse_audience(&v.audience),
        nudge: v.nudge,
        created_at: v.created_at,
        result,
    })
}

fn parse_audience(s: &str) -> Audience {
    Audience::from_str(s).unwrap_or(Audience::Unknown)
}

fn empty_fingerprint() -> StyleFingerprint {
    StyleFingerprint {
        formality_score: 0.0,
        avg_sentence_length: 0.0,
        sentence_length_variance: 0.0,
        exclamation_ratio: 0.0,
        ellipsis_ratio: 0.0,
        emoji_frequency: 0.0,
        contraction_ratio: 0.0,
        hedging_ratio: 0.0,
        common_openers: Vec::new(),
        common_closers: Vec::new(),
        message_count: 0,
    }
}
