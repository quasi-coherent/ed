//! Client services for ed-api.
use futures::future::BoxFuture;
use std::fmt::{self, Debug, Formatter};
use std::ops::Deref;
use std::sync::Arc;

mod claude;
mod embeddings;
mod service;
pub use service::{EmbeddingsClient, MessagesClient};
mod share;
pub use share::{EdHttpClient, EdHttpConfig};

pub mod types {
    pub use super::service::{EmbeddingsRequest, EmbeddingsResponse};
    pub use super::service::{Message, MessagesRequest, MessagesResponse};
}

/// A type that has the full query capability.
pub trait EdApiClient:
    EmbeddingsClient + MessagesClient + Send + Sync + 'static
{
}

impl<C> EdApiClient for C where
    C: EmbeddingsClient + MessagesClient + Send + Sync + 'static
{
}

/// `EdClientService` encapsulates the API methods required by our API handlers.
#[derive(Clone)]
pub struct EdClientService(Arc<dyn EdApiClient>);

impl EdClientService {
    /// New dynamically typed client service.
    pub fn new<C: EdApiClient>(client: C) -> Self {
        Self(Arc::new(client))
    }
}

impl Debug for EdClientService {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("EdClientService").finish()
    }
}

impl Deref for EdClientService {
    type Target = Arc<dyn EdApiClient>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Implements `EdClientService`.
#[derive(Clone, Debug)]
pub struct EdClientServiceImpl {
    embeddings: EdHttpClient,
    messages: EdHttpClient,
}

impl EdClientServiceImpl {
    pub fn new(embeddings: EdHttpClient, messages: EdHttpClient) -> Self {
        Self { embeddings, messages }
    }
}

impl EmbeddingsClient for EdClientServiceImpl {
    fn embed<'p, 'a: 'p>(
        &'a self,
        request: types::EmbeddingsRequest<'p>,
    ) -> BoxFuture<'p, anyhow::Result<types::EmbeddingsResponse>> {
        self.embeddings.embed(request)
    }
}

impl MessagesClient for EdClientServiceImpl {
    fn complete<'p, 'a: 'p>(
        &'a self,
        message: types::Message<'p>,
    ) -> BoxFuture<'p, anyhow::Result<types::MessagesResponse>> {
        self.messages.complete(message)
    }
}
