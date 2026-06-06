//! Client services for ed-api.
use futures::future::BoxFuture;
use secrecy::SecretString;
use std::fmt::{self, Debug, Formatter};
use std::ops::Deref;
use std::sync::Arc;

use crate::embeddings::*;
use crate::messages::*;

mod client;
pub use client::{EdHttpClient, EdHttpConfig};
mod messages;
pub use messages::{MessagesClient, RequestCompletion};
mod embeddings;
pub use embeddings::{EmbeddingsClient, RequestEmbedding};

pub mod types {
    pub use super::embeddings::{EmbeddingsRequest, EmbeddingsResponse};
    pub use super::messages::{MessagesRequest, MessagesResponse};
}

/// A type that has the full query capability.
pub trait ClientRepository:
    RequestEmbedding + RequestCompletion + Send + Sync + 'static
{
}

impl<C, T> ClientRepository for C
where
    T: RequestEmbedding + RequestCompletion + Send + Sync + 'static,
    C: Deref<Target = T> + Send + Sync + 'static,
{
}

/// Implements the client services.
#[derive(Clone)]
pub struct EdServices(Arc<dyn ClientRepository>);

/// Newtype wrappers only to disambiguate in try_init:
pub struct AnthropicToken(pub SecretString);
pub struct OpenAiToken(pub SecretString);

impl EdServices {
    /// New dynamically typed `ClientRepository`.
    pub fn new<C: ClientRepository>(repo: C) -> Self {
        Self(Arc::new(repo))
    }

    /// Initialize with defaults.
    pub fn try_init(
        anthropic: AnthropicToken,
        openai: OpenAiToken,
    ) -> anyhow::Result<Self> {
        let inner = reqwest::Client::new();

        let mcfg = EdHttpConfig::configure_messages();
        let ecfg = EdHttpConfig::configure_embeddings();

        let prompt = mcfg.try_into_client(&inner, anthropic.0)?;
        let embed = ecfg.try_into_client(&inner, openai.0)?;
        let repo = EdServicesInner::new(embed, prompt);

        Ok(Self::new(repo))
    }

    /// New from individual components.
    pub fn new_from<E, M>(embed: E, prompt: M) -> Self
    where
        E: RequestEmbedding + 'static,
        M: RequestCompletion + 'static,
    {
        let repo = EdServicesInner::new(embed, prompt);
        Self::new(repo)
    }
}

impl Deref for EdServices {
    type Target = Arc<dyn ClientRepository>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for EdServices {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("EdServices").finish()
    }
}

// Inner type for `EdServices`.
#[derive(Debug)]
struct EdServicesInner {
    embed: EmbeddingsClient,
    prompt: MessagesClient,
}

impl EdServicesInner {
    fn new<E, M>(embeddings: E, messages: M) -> Self
    where
        E: RequestEmbedding + 'static,
        M: RequestCompletion + 'static,
    {
        Self {
            embed: EmbeddingsClient::new(embeddings),
            prompt: MessagesClient::new(messages),
        }
    }
}

impl RequestEmbedding for EdServicesInner {
    fn embed<'p, 'a: 'p>(
        &'a self,
        request: EmbeddingsRequest<'p>,
    ) -> BoxFuture<'p, anyhow::Result<EmbeddingsResponse>> {
        self.embed.embed(request)
    }
}

impl RequestCompletion for EdServicesInner {
    fn prompt<'p, 'a: 'p>(
        &'a self,
        request: MessagesRequest<'p>,
    ) -> BoxFuture<'p, anyhow::Result<MessagesResponse>> {
        self.prompt.prompt(request)
    }
}

impl ClientRepository for EdServicesInner {}
