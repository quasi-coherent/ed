use anyhow::Context as _;
use futures::future::BoxFuture;

use crate::service::*;
use crate::share::{EdHttpClient, EdHttpConfig};

const ENDPOINT: &str = "https://api.openai.com/v1/embeddings";
const MODEL: &str = "text-embedding-3-small";
const OPENAPI_TOKEN_KEY: &str = "OPENAI_API_KEY";

impl EdHttpClient {
    /// With defaults for the embeddings API.
    pub fn embeddings_default(inner: &reqwest::Client) -> anyhow::Result<Self> {
        EdHttpConfig::new(ENDPOINT)
            .with_model(MODEL)
            .new_with_bearer(inner, OPENAPI_TOKEN_KEY)
    }
}

impl EmbeddingsClient for EdHttpClient {
    fn embed<'p, 'a: 'p>(
        &'a self,
        request: EmbeddingsRequest<'p>,
    ) -> BoxFuture<'p, anyhow::Result<EmbeddingsResponse>> {
        Box::pin(async move {
            let response = self.post_builder(&request).send().await?;

            let status = response.status();
            let bytes = &response.bytes().await?;

            if !status.is_success() {
                let body = String::from_utf8_lossy(bytes).into_owned();
                return Err(anyhow::anyhow!("{body}"));
            }

            serde_json::from_slice::<EmbeddingsResponseRaw>(bytes)
                .context("embeddings response ser/de failed")
                .and_then(EmbeddingsResponse::try_from)
        })
    }
}
