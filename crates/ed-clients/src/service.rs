use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};
use std::convert::AsRef;
use std::ops::Deref;

/// Type that can use the Anthropic messages API.
pub trait MessagesClient {
    /// Send the request and interpret the response.
    fn complete<'p, 'a: 'p>(
        &'a self,
        message: Message<'p>,
    ) -> BoxFuture<'p, anyhow::Result<MessagesResponse>>;
}

/// The request body is shaped this way.
#[derive(Debug, Serialize)]
pub struct MessagesRequest<'a> {
    model: &'a str,
    max_tokens: u32,
    messages: [Message<'a>; 1],
}

/// A `Message` in the request to the messages API.
#[derive(Debug, Serialize)]
pub struct Message<'a> {
    role: &'a str,
    content: &'a str,
}

impl<'a> Message<'a> {
    /// New message with "user" role.
    pub fn new_default<P: AsRef<str>>(prompt: &'a P) -> Message<'a> {
        Message { role: "user", content: prompt.as_ref() }
    }

    /// New `Message`.
    pub fn new<P: AsRef<str>>(role: &'a str, prompt: &'a P) -> Message<'a> {
        Message { role, content: prompt.as_ref() }
    }

    /// Turn into a request object.
    pub fn into_request(
        self,
        model: &'a str,
        max_tokens: u32,
    ) -> MessagesRequest<'a> {
        MessagesRequest { model, max_tokens, messages: [self] }
    }
}

#[derive(Clone, Deserialize)]
struct ContentBlock {
    #[serde(rename = "type")]
    _kind: String,
    text: String,
}

#[derive(Clone, Deserialize)]
pub(crate) struct MessagesResponseRaw {
    content: Vec<ContentBlock>,
}

/// Parsed response from the Anthropic messages API.
#[derive(Clone, Debug)]
pub struct MessagesResponse(String);

impl Deref for MessagesResponse {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<MessagesResponseRaw> for MessagesResponse {
    type Error = anyhow::Error;

    fn try_from(value: MessagesResponseRaw) -> Result<Self, Self::Error> {
        value
            .content
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("missing content[0]"))
            .map(|v| Self(v.text))
    }
}

/// Type that can use OpenAI embeddings API.
pub trait EmbeddingsClient {
    /// Make the request and interpret the response.
    fn embed<'p, 'a: 'p>(
        &'a self,
        request: EmbeddingsRequest<'p>,
    ) -> BoxFuture<'p, anyhow::Result<EmbeddingsResponse>>;
}

#[derive(Serialize)]
pub struct EmbeddingsRequest<'a> {
    model: &'a str,
    input: &'a str,
}

impl<'a> EmbeddingsRequest<'a> {
    /// New request.
    pub fn new(model: &'a str, input: &'a str) -> EmbeddingsRequest<'a> {
        Self { model, input }
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

/// Parsed response from the OpenAI embeddings API.
#[derive(Clone, Debug)]
pub struct EmbeddingsResponse(Vec<f32>);

impl Deref for EmbeddingsResponse {
    type Target = [f32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
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
