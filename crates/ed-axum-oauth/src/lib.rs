use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{Router, get};
use ed_db::EdUserTable;
use secrecy::SecretString;
use std::fmt::Display;
use std::sync::Arc;

mod client;
use client::OAuthClient;
mod handler;
pub use handler::AuthHandler;
mod provider;
pub use provider::Provider;

pub(crate) mod constants {
    pub(crate) const COOKIE_AUTH_SESSION: &str = "auth_session";
    pub(crate) const COOKIE_AUTH_CSRF_STATE: &str = "auth_csrf_state";
    pub(crate) const COOKIE_AUTH_CODE_VERIFIER: &str = "auth_code_verifier";
    pub(crate) const COOKIE_AUTH_DURATION: time::Duration =
        time::Duration::milliseconds(1000 * 60);
    pub(crate) const SESSION_DURATION: time::Duration =
        time::Duration::milliseconds(1000 * 60 * 60 * 24);
}

pub mod providers {
    pub use super::provider::Google;
}

/// `State` value for an `axum` router handling oauth2 flows.
#[derive(Clone)]
pub struct AuthState {
    db: Arc<EdUserTable>,
    client_id: SecretString,
    client_secret: SecretString,
    redirect_url: String,
}

impl AuthState {
    pub async fn new(
        redirect_url: String,
        client_id: SecretString,
        client_secret: SecretString,
        db_url: &SecretString,
    ) -> anyhow::Result<Self> {
        let db = EdUserTable::try_init(db_url).await?;
        Ok(Self {
            db: Arc::new(db),
            client_id,
            client_secret,
            redirect_url,
        })
    }

    fn oauth_client<P: Provider>(&self) -> Result<OAuthClient<P>, AuthError>
    {
        let redirect_url = url::Url::parse(&self.redirect_url)?;
        Ok(OAuthClient::new(
            &self.client_id,
            &self.client_secret,
            redirect_url,
            P::default(),
        ))
    }

    /// Router to handle oauth2 flows with the provider `P`.
    pub fn router<P: Provider + 'static>() -> Router<AuthState>
    {
        Router::new()
            .route("/login", get(handler::AuthHandler::<P>::handle_login))
            .route("/callback", get(handler::AuthHandler::<P>::handle_callback))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("invalid state")]
    InvalidState,
    #[error("token exchange failed: {0}")]
    Exchange(String),
    #[error("userinfo fetch failed: {0}")]
    Userinfo(String),
    #[error("internal error: {0}")]
    Internal(Box<dyn std::error::Error + Send + Sync>),
}

impl AuthError {
    fn invalid() -> Self {
        Self::InvalidState
    }

    fn exchange<E: Display>(e: E) -> Self {
        Self::Exchange(e.to_string())
    }

    fn userinfo<E: Display>(e: E) -> Self {
        Self::Userinfo(e.to_string())
    }

    fn internal<E: std::error::Error + Send + Sync + 'static>(e: E) -> Self {
        Self::Internal(Box::new(e))
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            Self::Internal(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            },
            Self::InvalidState => (StatusCode::UNAUTHORIZED, self.to_string()),
            Self::Exchange(e) | Self::Userinfo(e) => {
                (StatusCode::UNAUTHORIZED, e.to_string())
            },
        }
        .into_response()
    }
}

impl From<url::ParseError> for AuthError {
    fn from(value: url::ParseError) -> Self {
        Self::Internal(Box::new(value))
    }
}

impl From<reqwest::Error> for AuthError {
    fn from(value: reqwest::Error) -> Self {
        Self::Internal(Box::new(value))
    }
}

impl From<anyhow::Error> for AuthError {
    fn from(value: anyhow::Error) -> Self {
        Self::Internal(value.into_boxed_dyn_error())
    }
}
