use anyhow::Context as _;
use futures::future::BoxFuture;
use secrecy::ExposeSecret as _;
use serde::{Deserialize, Serialize};
use std::convert::AsRef;
use std::fmt::{self, Debug, Formatter};
use std::ops::Deref;

use crate::client::EdHttpClient;

const DEFAULT_MODEL: &str = "claude-sonnet-4-6";
const DEFAULT_MAX_TOKENS: u32 = 1024;

/// Type that can use the Anthropic messages API to prompt.
pub trait RequestCompletion: Send + Sync {
    /// Send the request and interpret the response.
    fn prompt<'p, 'a: 'p>(
        &'a self,
        request: MessagesRequest<'p>,
    ) -> BoxFuture<'p, anyhow::Result<MessagesResponse>>;
}

impl RequestCompletion for EdHttpClient {
    fn prompt<'p, 'a: 'p>(
        &'a self,
        request: MessagesRequest<'p>,
    ) -> BoxFuture<'p, anyhow::Result<MessagesResponse>> {
        Box::pin(async move {
            self.post_with_secret(|secret, builder| {
                builder
                    .header("x-api-key", secret.expose_secret())
                    .json(&request)
            })
            .send()
            .await?
            .json::<MessagesResponseRaw>()
            .await
            .context("error deserializing response")
            .and_then(MessagesResponse::try_from)
        })
    }
}

impl<C, T> RequestCompletion for C
where
    C: Deref<Target = T> + Send + Sync,
    for<'c> T: RequestCompletion + 'c,
{
    fn prompt<'p, 'a: 'p>(
        &'a self,
        request: MessagesRequest<'p>,
    ) -> BoxFuture<'p, anyhow::Result<MessagesResponse>> {
        self.deref().prompt(request)
    }
}

/// Dynamic `RequestCompletion`.
pub struct MessagesClient(Box<dyn RequestCompletion>);

impl MessagesClient {
    /// New dynamically typed client service.
    pub fn new<C: RequestCompletion + 'static>(client: C) -> Self {
        Self(Box::new(client))
    }
}

impl Debug for MessagesClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("MessagesClient").finish()
    }
}

impl Deref for MessagesClient {
    type Target = Box<dyn RequestCompletion>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The request body is shaped this way.
#[derive(Debug, Serialize)]
pub struct MessagesRequest<'p> {
    model: &'p str,
    max_tokens: u32,
    messages: [Message<'p>; 1],
}

impl<'p> MessagesRequest<'p> {
    /// New Anthropic messages API request.
    pub fn new<P: AsRef<str>>(prompt: &'p P) -> MessagesRequest<'p> {
        let message = Message::new(prompt);
        MessagesRequest {
            model: DEFAULT_MODEL,
            max_tokens: DEFAULT_MAX_TOKENS,
            messages: [message],
        }
    }

    /// Set the model to use.
    pub fn with_model(self, model: &'p str) -> MessagesRequest<'p> {
        MessagesRequest {
            model,
            max_tokens: self.max_tokens,
            messages: self.messages,
        }
    }

    /// Set the model to use.
    pub fn with_max_tokens(self, max_tokens: u32) -> MessagesRequest<'p> {
        MessagesRequest {
            model: self.model,
            max_tokens,
            messages: self.messages,
        }
    }
}

/// Parsed response from the Anthropic messages API.
#[derive(Clone, Debug)]
pub struct MessagesResponse(pub(crate) String);

impl Deref for MessagesResponse {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Serialize)]
struct Message<'a> {
    role: &'a str,
    content: &'a str,
}

impl<'a> Message<'a> {
    fn new<P: AsRef<str>>(prompt: &'a P) -> Message<'a> {
        Message { role: "user", content: prompt.as_ref() }
    }
}

#[derive(Clone, Deserialize)]
struct ContentBlock {
    #[serde(rename = "type")]
    _kind: String,
    text: String,
}

#[derive(Clone, Deserialize)]
struct MessagesResponseRaw {
    content: Vec<ContentBlock>,
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
