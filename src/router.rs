//! Glues the generated `Default` handler trait to our per-route bodies.

use async_trait::async_trait;
use axum_extra::extract::CookieJar;
use ed_axum::apis::ErrorHandler;
use ed_axum::apis::default::{
    DataUserIdDeleteResponse, DataUserIdFingerprintGetResponse,
    DataUserIdGetResponse, DataUserIdPostResponse, Default,
    SimulateUserIdGetResponse, SimulateUserIdPostResponse,
    SimulateUserIdSimulationsSimulationIdDeleteResponse,
    SimulateUserIdSimulationsSimulationIdGetResponse,
};
use ed_axum::models;
use headers::Host;
use http::Method;

use crate::error::AppError;
use crate::handlers;
use crate::state::AppState;

impl ErrorHandler<()> for AppState {}

#[async_trait]
impl Default<()> for AppState {
    async fn data_user_id_delete(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        path: &models::DataUserIdDeletePathParams,
    ) -> Result<DataUserIdDeleteResponse, ()> {
        Ok(match handlers::delete_corpus(self, path.user_id).await {
            Ok(()) => DataUserIdDeleteResponse::Status204_CorpusDeleted,
            Err(e) => DataUserIdDeleteResponse::Status500_InternalServerError(
                e.body(),
            ),
        })
    }

    async fn data_user_id_fingerprint_get(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        path: &models::DataUserIdFingerprintGetPathParams,
    ) -> Result<DataUserIdFingerprintGetResponse, ()> {
        Ok(match handlers::get_fingerprint(self, path.user_id).await {
            Ok(fp) => {
                DataUserIdFingerprintGetResponse::Status200_StyleFingerprint(fp)
            },
            Err(AppError::NotFound) => {
                DataUserIdFingerprintGetResponse::Status404_NoCorpusHasBeenIngestedYet
            },
            Err(e) => {
                DataUserIdFingerprintGetResponse::Status500_InternalServerError(
                    e.body(),
                )
            },
        })
    }

    async fn data_user_id_get(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        path: &models::DataUserIdGetPathParams,
        query: &models::DataUserIdGetQueryParams,
    ) -> Result<DataUserIdGetResponse, ()> {
        let result = handlers::list_corpus(
            self,
            path.user_id,
            query.audience,
            query.limit,
            query.offset,
        )
        .await;
        Ok(match result {
            Ok(list) => {
                DataUserIdGetResponse::Status200_PaginatedListOfCorpusMessages(
                    list,
                )
            },
            Err(e) => {
                DataUserIdGetResponse::Status500_InternalServerError(e.body())
            },
        })
    }

    async fn data_user_id_post(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        path: &models::DataUserIdPostPathParams,
        body: &models::CorpusUpload,
    ) -> Result<DataUserIdPostResponse, ()> {
        let result = handlers::ingest(self, path.user_id, body).await;
        Ok(match result {
            Ok(r) => {
                DataUserIdPostResponse::Status200_CorpusSuccessfullyIngested(r)
            },
            Err(AppError::InvalidInput(m)) => {
                DataUserIdPostResponse::Status422_RequestBodyFailedValidation(
                    models::ErrorBody { error: m },
                )
            },
            Err(e) => {
                DataUserIdPostResponse::Status500_InternalServerError(e.body())
            },
        })
    }

    async fn simulate_user_id_get(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        path: &models::SimulateUserIdGetPathParams,
        query: &models::SimulateUserIdGetQueryParams,
    ) -> Result<SimulateUserIdGetResponse, ()> {
        let result = handlers::list_simulations(
            self,
            path.user_id,
            query.limit,
            query.offset,
        )
        .await;
        Ok(match result {
            Ok(list) => {
                SimulateUserIdGetResponse::Status200_PaginatedListOfPastGenerations(
                    list,
                )
            },
            Err(e) => SimulateUserIdGetResponse::Status500_InternalServerError(
                e.body(),
            ),
        })
    }

    async fn simulate_user_id_post(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        path: &models::SimulateUserIdPostPathParams,
        body: &models::GenerateRequest,
    ) -> Result<SimulateUserIdPostResponse, ()> {
        let result = handlers::generate(self, path.user_id, body).await;
        Ok(match result {
            Ok(r) => {
                SimulateUserIdPostResponse::Status200_GeneratedTextWithSupportingContextAndConfidenceScore(r)
            },
            Err(AppError::NotFound) => {
                SimulateUserIdPostResponse::Status404_NoCorpusHasBeenIngestedYet
            },
            Err(AppError::InvalidInput(m)) => {
                SimulateUserIdPostResponse::Status422_RequestBodyFailedValidation(
                    models::ErrorBody { error: m },
                )
            },
            Err(e) => SimulateUserIdPostResponse::Status500_InternalServerError(
                e.body(),
            ),
        })
    }

    async fn simulate_user_id_simulations_simulation_id_delete(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        path: &models::SimulateUserIdSimulationsSimulationIdDeletePathParams,
    ) -> Result<SimulateUserIdSimulationsSimulationIdDeleteResponse, ()> {
        let result =
            handlers::delete_simulation(self, path.user_id, path.simulation_id)
                .await;
        Ok(match result {
            Ok(()) => {
                SimulateUserIdSimulationsSimulationIdDeleteResponse::Status204_GenerationDeleted
            },
            Err(AppError::NotFound) => {
                SimulateUserIdSimulationsSimulationIdDeleteResponse::Status404_NoGenerationFoundWithThatID
            },
            Err(e) => SimulateUserIdSimulationsSimulationIdDeleteResponse::Status500_InternalServerError(
                e.body(),
            ),
        })
    }

    async fn simulate_user_id_simulations_simulation_id_get(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        path: &models::SimulateUserIdSimulationsSimulationIdGetPathParams,
    ) -> Result<SimulateUserIdSimulationsSimulationIdGetResponse, ()> {
        let result =
            handlers::get_simulation(self, path.user_id, path.simulation_id)
                .await;
        Ok(match result {
            Ok(entry) => {
                SimulateUserIdSimulationsSimulationIdGetResponse::Status200_ASinglePersistedGenerationResult(entry)
            },
            Err(AppError::NotFound) => {
                SimulateUserIdSimulationsSimulationIdGetResponse::Status404_NoGenerationFoundWithThatID
            },
            Err(e) => SimulateUserIdSimulationsSimulationIdGetResponse::Status500_InternalServerError(
                e.body(),
            ),
        })
    }
}
