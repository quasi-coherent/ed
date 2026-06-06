use reqwest::RequestBuilder;
use reqwest::header::{HeaderMap, HeaderValue, IntoHeaderName};
use secrecy::{ExposeSecret, SecretString};

const MESSAGES_ENDPOINT: &str = "https://api.anthropic.com/v1/messages";
const ANTHROPIC_VERSION: &str = "2023-06-01";
const DEFAULT_MESSAGES_SECRET_KEY: &str = "MESSAGES_API_TOKEN";

const EMBEDDINGS_ENDPOINT: &str = "https://api.openai.com/v1/embeddings";
const DEFAULT_EMBEDDINGS_SECRET_KEY: &str = "EMBEDDINGS_API_TOKEN";

/// Config for building settings.
#[derive(Clone, Debug)]
pub struct EdHttpConfig {
    uri: String,
    default_headers: HeaderMap,
    empty: String,
}

impl EdHttpConfig {
    fn new<T: Into<String>>(uri: T) -> Self {
        Self {
            uri: uri.into(),
            default_headers: Default::default(),
            empty: String::default(),
        }
    }

    /// Default configuration for the messages API.
    pub fn configure_messages() -> Self {
        Self::new(MESSAGES_ENDPOINT).add_default_header(
            "anthropic-version",
            HeaderValue::from_static(ANTHROPIC_VERSION),
        )
    }

    /// Default configuration for the embeddings API.
    pub fn configure_embeddings() -> Self {
        Self::new(EMBEDDINGS_ENDPOINT)
    }

    /// Add an HTTP header to be set on every request.
    pub fn add_default_header<K: IntoHeaderName, V: Into<HeaderValue>>(
        mut self,
        k: K,
        v: V,
    ) -> Self {
        self.default_headers.append(k, v.into());
        self
    }

    /// Create an http client with the inner reqest client from this config.
    pub fn try_into_client(
        self,
        inner: &reqwest::Client,
        secret: SecretString,
    ) -> anyhow::Result<EdHttpClient> {
        Ok(EdHttpClient {
            inner: inner.clone(),
            uri: url::Url::parse(&self.uri)?,
            default_headers: self.default_headers,
            secret,
        })
    }
}

/// An HTTP client.
#[derive(Clone, Debug)]
pub struct EdHttpClient {
    pub(crate) inner: reqwest::Client,
    pub(crate) uri: url::Url,
    pub(crate) default_headers: HeaderMap,
    pub(crate) secret: SecretString,
}

impl EdHttpClient {
    pub(crate) fn post_with_secret<F>(&self, mut f: F) -> RequestBuilder
    where
        F: FnMut(&SecretString, RequestBuilder) -> RequestBuilder,
    {
        let base = self
            .inner
            .post(self.uri.clone())
            .headers(self.default_headers.clone());
        f(&self.secret, base)
    }
}
