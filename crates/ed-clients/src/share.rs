use anyhow::Context as _;
use reqwest::RequestBuilder;
use reqwest::header::{HeaderMap, HeaderValue, IntoHeaderName};
use secrecy::{ExposeSecret as _, SecretString};
use serde::Serialize;
use std::convert::AsRef;

/// Config for building settings.
#[derive(Clone, Debug)]
pub struct EdHttpConfig {
    uri: String,
    default_headers: HeaderMap,
    model: String,
    max_tokens: Option<u32>,
}

impl EdHttpConfig {
    /// New with defaults.
    pub fn new<T: Into<String>>(uri: T) -> Self {
        Self {
            uri: uri.into(),
            default_headers: Default::default(),
            model: Default::default(),
            max_tokens: None,
        }
    }

    /// Set the model to use.
    pub fn with_model<T: Into<String>>(self, model: T) -> Self {
        Self { model: model.into(), ..self }
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

    /// Maximum allowed tokens per request.
    pub fn set_max_token_use(self, max_tokens: u32) -> Self {
        Self { max_tokens: Some(max_tokens), ..self }
    }

    /// Build settings given an environment variable key where a bearer token is
    /// expected to be found.
    pub fn new_with_bearer<K: AsRef<str>>(
        self,
        inner: &reqwest::Client,
        var_key: K,
    ) -> anyhow::Result<EdHttpClient> {
        let uri = url::Url::parse(&self.uri)?;
        let bearer = std::env::var(var_key.as_ref())
            .map(SecretString::from)
            .context("missing bearer token")?;
        Ok(EdHttpClient {
            inner: inner.clone(),
            uri,
            default_headers: self.default_headers,
            model: self.model,
            max_tokens: self.max_tokens,
            bearer,
        })
    }
}

/// An HTTP client.
#[derive(Clone, Debug)]
pub struct EdHttpClient {
    pub(crate) inner: reqwest::Client,
    pub(crate) uri: url::Url,
    pub(crate) default_headers: HeaderMap,
    pub(crate) model: String,
    pub(crate) bearer: SecretString,
    pub(crate) max_tokens: Option<u32>,
}

impl EdHttpClient {
    /// New given settings.
    pub fn try_new<K: AsRef<str>>(
        inner: &reqwest::Client,
        config: EdHttpConfig,
        var_key: K,
    ) -> anyhow::Result<Self> {
        config.new_with_bearer(inner, var_key)
    }

    /// Return a pre-configured POST request.
    pub fn post_builder<B: Serialize>(&self, request: &B) -> RequestBuilder {
        self.inner
            .post(self.uri.clone())
            .headers(self.default_headers.clone())
            .bearer_auth(self.bearer.expose_secret())
            .json(request)
    }

    /// Return a pre-configured POST request where the auth token goes in an atypical header.
    pub fn post_builder_with_auth<K: IntoHeaderName, B: Serialize>(
        &self,
        request: &B,
        auth_header_key: K,
    ) -> anyhow::Result<RequestBuilder> {
        let mut headers = self.default_headers.clone();
        headers.append(
            auth_header_key,
            HeaderValue::from_str(self.bearer.expose_secret())
                .context("header value from token")?,
        );

        Ok(self.inner.post(self.uri.clone()).headers(headers).json(request))
    }
}
