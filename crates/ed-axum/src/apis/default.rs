use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::CookieJar;
use bytes::Bytes;
use headers::Host;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DataUserIdDeleteResponse {
    /// Corpus deleted
    Status204_CorpusDeleted,
    /// Internal server error
    Status500_InternalServerError(models::ErrorBody),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DataUserIdFingerprintGetResponse {
    /// Style fingerprint
    Status200_StyleFingerprint(models::StyleFingerprint),
    /// No corpus has been ingested yet
    Status404_NoCorpusHasBeenIngestedYet,
    /// Internal server error
    Status500_InternalServerError(models::ErrorBody),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DataUserIdGetResponse {
    /// Paginated list of corpus messages
    Status200_PaginatedListOfCorpusMessages(models::MessageList),
    /// Internal server error
    Status500_InternalServerError(models::ErrorBody),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DataUserIdPostResponse {
    /// Corpus successfully ingested
    Status200_CorpusSuccessfullyIngested(models::IngestResult),
    /// Request body failed validation
    Status422_RequestBodyFailedValidation(models::ErrorBody),
    /// Internal server error
    Status500_InternalServerError(models::ErrorBody),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum SimulateUserIdGetResponse {
    /// Paginated list of past generations
    Status200_PaginatedListOfPastGenerations(models::HistoryList),
    /// Internal server error
    Status500_InternalServerError(models::ErrorBody),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum SimulateUserIdPostResponse {
    /// Generated text with supporting context and confidence score
    Status200_GeneratedTextWithSupportingContextAndConfidenceScore(
        models::GenerateResponse,
    ),
    /// No corpus has been ingested yet
    Status404_NoCorpusHasBeenIngestedYet,
    /// Request body failed validation
    Status422_RequestBodyFailedValidation(models::ErrorBody),
    /// Internal server error
    Status500_InternalServerError(models::ErrorBody),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum SimulateUserIdSimulationsSimulationIdDeleteResponse {
    /// Generation deleted
    Status204_GenerationDeleted,
    /// No generation found with that ID
    Status404_NoGenerationFoundWithThatID,
    /// Internal server error
    Status500_InternalServerError(models::ErrorBody),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum SimulateUserIdSimulationsSimulationIdGetResponse {
    /// A single persisted generation result
    Status200_ASinglePersistedGenerationResult(models::SimulationEntry),
    /// No generation found with that ID
    Status404_NoGenerationFoundWithThatID,
    /// Internal server error
    Status500_InternalServerError(models::ErrorBody),
}

/// Default
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Default<E: std::fmt::Debug + Send + Sync + 'static = ()>:
    super::ErrorHandler<E>
{
    /// Delete the stored corpus.
    ///
    /// DataUserIdDelete - DELETE /data/{user_id}
    async fn data_user_id_delete(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::DataUserIdDeletePathParams,
    ) -> Result<DataUserIdDeleteResponse, E>;

    /// Retrieve the style fingerprint.
    ///
    /// DataUserIdFingerprintGet - GET /data/{user_id}/fingerprint
    async fn data_user_id_fingerprint_get(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::DataUserIdFingerprintGetPathParams,
    ) -> Result<DataUserIdFingerprintGetResponse, E>;

    /// List stored corpus messages.
    ///
    /// DataUserIdGet - GET /data/{user_id}
    async fn data_user_id_get(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::DataUserIdGetPathParams,
        query_params: &models::DataUserIdGetQueryParams,
    ) -> Result<DataUserIdGetResponse, E>;

    /// Ingest a corpus of text.
    ///
    /// DataUserIdPost - POST /data/{user_id}
    async fn data_user_id_post(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::DataUserIdPostPathParams,
        body: &models::CorpusUpload,
    ) -> Result<DataUserIdPostResponse, E>;

    /// List past generations.
    ///
    /// SimulateUserIdGet - GET /simulate/{user_id}
    async fn simulate_user_id_get(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::SimulateUserIdGetPathParams,
        query_params: &models::SimulateUserIdGetQueryParams,
    ) -> Result<SimulateUserIdGetResponse, E>;

    /// Generate text matching the corpus voice.
    ///
    /// SimulateUserIdPost - POST /simulate/{user_id}
    async fn simulate_user_id_post(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::SimulateUserIdPostPathParams,
        body: &models::GenerateRequest,
    ) -> Result<SimulateUserIdPostResponse, E>;

    /// Delete a single past generation.
    ///
    /// SimulateUserIdSimulationsSimulationIdDelete - DELETE /simulate/{user_id}/simulations/{simulation_id}
    async fn simulate_user_id_simulations_simulation_id_delete(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::SimulateUserIdSimulationsSimulationIdDeletePathParams,
    ) -> Result<SimulateUserIdSimulationsSimulationIdDeleteResponse, E>;

    /// Retrieve a single past generation.
    ///
    /// SimulateUserIdSimulationsSimulationIdGet - GET /simulate/{user_id}/simulations/{simulation_id}
    async fn simulate_user_id_simulations_simulation_id_get(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::SimulateUserIdSimulationsSimulationIdGetPathParams,
    ) -> Result<SimulateUserIdSimulationsSimulationIdGetResponse, E>;
}
