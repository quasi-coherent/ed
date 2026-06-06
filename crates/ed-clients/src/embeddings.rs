use anyhow::Context as _;
use futures::future::BoxFuture;
use secrecy::ExposeSecret as _;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Formatter};
use std::ops::Deref;

use crate::client::EdHttpClient;

const DEFAULT_MODEL: &str = "text-embedding-3-small";

/// Type that can use OpenAI embeddings API.
pub trait RequestEmbedding: Send + Sync {
    /// Make the request and interpret the response.
    fn embed<'p, 'a: 'p>(
        &'a self,
        request: EmbeddingsRequest<'p>,
    ) -> BoxFuture<'p, anyhow::Result<EmbeddingsResponse>>;
}

impl RequestEmbedding for EdHttpClient {
    fn embed<'p, 'a: 'p>(
        &'a self,
        request: EmbeddingsRequest<'p>,
    ) -> BoxFuture<'p, anyhow::Result<EmbeddingsResponse>> {
        Box::pin(async move {
            self.post_with_secret(|secret, builder| {
                builder.bearer_auth(secret.expose_secret()).json(&request)
            })
            .send()
            .await?
            .json::<EmbeddingsResponseRaw>()
            .await
            .context("serde")
            .and_then(EmbeddingsResponse::try_from)
        })
    }
}

impl<C, T> RequestEmbedding for C
where
    C: Deref<Target = T> + Send + Sync,
    for<'c> T: RequestEmbedding + 'c,
{
    fn embed<'p, 'a: 'p>(
        &'a self,
        request: EmbeddingsRequest<'p>,
    ) -> BoxFuture<'p, anyhow::Result<EmbeddingsResponse>> {
        self.deref().embed(request)
    }
}

/// Dynamic `RequestEmbedding`.
pub struct EmbeddingsClient(Box<dyn RequestEmbedding>);

impl EmbeddingsClient {
    /// New dynamically typed client service.
    pub fn new<C: RequestEmbedding + 'static>(client: C) -> Self {
        Self(Box::new(client))
    }
}

impl Debug for EmbeddingsClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("EmbeddingsClient").finish()
    }
}

impl Deref for EmbeddingsClient {
    type Target = Box<dyn RequestEmbedding>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Request to use `model` to calculate a vector embedding of `input`.
#[derive(Serialize)]
pub struct EmbeddingsRequest<'a> {
    model: &'a str,
    input: &'a str,
}

impl<'a> EmbeddingsRequest<'a> {
    /// New request.
    pub fn new(input: &'a str) -> EmbeddingsRequest<'a> {
        EmbeddingsRequest { model: DEFAULT_MODEL, input }
    }

    /// Set the model to use in the request.
    pub fn with_model(self, model: &'a str) -> EmbeddingsRequest<'a> {
        EmbeddingsRequest { model, input: self.input }
    }
}

/// Parsed response from the OpenAI embeddings API.
#[derive(Clone, Debug)]
pub struct EmbeddingsResponse(pub(crate) Vec<f32>);

impl Deref for EmbeddingsResponse {
    type Target = [f32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Deserialize)]
struct EmbeddingItem {
    embedding: Vec<f32>,
}

#[derive(Deserialize)]
pub(crate) struct EmbeddingsResponseRaw {
    data: Vec<EmbeddingItem>,
}

impl TryFrom<EmbeddingsResponseRaw> for EmbeddingsResponse {
    type Error = anyhow::Error;

    fn try_from(value: EmbeddingsResponseRaw) -> Result<Self, Self::Error> {
        value
            .data
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("missing data[0]"))
            .map(|v| Self(v.embedding))
    }
}
