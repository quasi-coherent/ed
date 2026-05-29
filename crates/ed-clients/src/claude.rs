use anyhow::Context as _;
use futures::future::BoxFuture;
use reqwest::header::HeaderValue;

use crate::service::*;
use crate::share::{EdHttpClient, EdHttpConfig};

const ENDPOINT: &str = "https://api.anthropic.com/v1/messages";
const MODEL: &str = "claude-sonnet-4-20250514";
const MAX_TOKENS: u32 = 1024;
const ANTHROPIC_VERSION: &str = "2023-06-01";
const ANTHROPIC_TOKEN_KEY: &str = "ANTHROPIC_API_KEY";

impl EdHttpClient {
    /// With defaults for the messages API.
    pub fn messages_default(inner: &reqwest::Client) -> anyhow::Result<Self> {
        EdHttpConfig::new(ENDPOINT)
            .with_model(MODEL)
            .add_default_header(
                "anthropic-version",
                HeaderValue::from_str(ANTHROPIC_VERSION)
                    .context("header value from token")?,
            )
            .set_max_token_use(MAX_TOKENS)
            .new_with_bearer(inner, ANTHROPIC_TOKEN_KEY)
    }
}

impl MessagesClient for EdHttpClient {
    fn complete<'p, 'a: 'p>(
        &'a self,
        message: Message<'p>,
    ) -> BoxFuture<'p, anyhow::Result<MessagesResponse>> {
        Box::pin(async move {
            let request = message.into_request(
                &self.model,
                self.max_tokens.unwrap_or(MAX_TOKENS),
            );

            let response = self
                .post_builder_with_auth(&request, "x-api-key")?
                .send()
                .await?;

            let status = response.status();
            let bytes = &response.bytes().await?;

            if !status.is_success() {
                let body = String::from_utf8_lossy(bytes).into_owned();
                return Err(anyhow::anyhow!("{body}"));
            }

            serde_json::from_slice::<MessagesResponseRaw>(&bytes)
                .context("messages response serde/failed")
                .and_then(MessagesResponse::try_from)
        })
    }
}
