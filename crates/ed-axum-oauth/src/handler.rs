use anyhow::Context as _;
use axum::Json;
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Redirect};
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::{Cookie, SameSite};
use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use serde::Serialize;
use std::ops::Deref;

use crate::client::AuthRequest;
use crate::constants::*;
use crate::provider::{Provider, Providers};
use crate::{AuthError, AuthState};

#[derive(Default)]
pub struct AuthCookies {
    inner: CookieJar,
}

impl AuthCookies {
    pub(crate) fn new(inner: CookieJar) -> Self {
        Self { inner }
    }

    pub(crate) fn new_cleared() -> Self {
        let mut clear_csrf = Cookie::new(COOKIE_AUTH_CSRF_STATE, "");
        clear_csrf.set_path("/");
        clear_csrf.make_removal();
        let mut clear_verifier = Cookie::new(COOKIE_AUTH_CODE_VERIFIER, "");
        clear_verifier.set_path("/");
        clear_verifier.make_removal();

        let inner = CookieJar::new().add(clear_csrf).add(clear_verifier);

        Self { inner }
    }

    pub(crate) fn set_session(self, session: UserSession) -> Self {
        let cookie = Cookie::build((
            COOKIE_AUTH_SESSION,
            session.session_id.to_string(),
        ))
        .http_only(true)
        .path("/")
        .same_site(SameSite::Lax)
        .max_age(SESSION_DURATION);
        Self { inner: self.inner.add(cookie) }
    }

    pub(crate) fn set_state(self, state: oauth2::CsrfToken) -> Self {
        let cookie =
            Cookie::build((COOKIE_AUTH_CSRF_STATE, state.secret().to_owned()))
                .http_only(true)
                .path("/")
                .same_site(SameSite::Lax)
                .max_age(COOKIE_AUTH_DURATION);
        Self { inner: self.inner.add(cookie) }
    }

    pub(crate) fn set_verifier(
        self,
        verifier: oauth2::PkceCodeVerifier,
    ) -> Self {
        let cookie = Cookie::build((
            COOKIE_AUTH_CODE_VERIFIER,
            verifier.secret().to_owned(),
        ))
        .http_only(true)
        .path("/")
        .same_site(SameSite::Lax)
        .max_age(COOKIE_AUTH_DURATION);
        Self { inner: self.inner.add(cookie) }
    }

    pub(crate) fn is_valid(&self, challenge: &str) -> bool {
        self.inner
            .get(COOKIE_AUTH_CSRF_STATE)
            .is_some_and(|ss| ss.value() == challenge)
    }

    pub(crate) fn try_get_verifier(
        &self,
    ) -> Result<oauth2::PkceCodeVerifier, AuthError> {
        self.inner
            .get(COOKIE_AUTH_CODE_VERIFIER)
            .ok_or_else(AuthError::invalid)
            .map(|sv| oauth2::PkceCodeVerifier::new(sv.value().to_owned()))
    }

    pub(crate) fn into_inner(self) -> CookieJar {
        self.inner
    }
}

pub struct AuthHandler<P = ()>(std::marker::PhantomData<P>);

impl AuthHandler {
    /// Get /auth
    pub async fn handle_get_providers() -> impl IntoResponse {
        (StatusCode::OK, Json(Providers::all()))
    }
}

impl<P: Provider + 'static> AuthHandler<P> {
    /// GET /auth/<provider>/login
    pub async fn handle_login(
        state: State<AuthState>,
    ) -> Result<impl IntoResponse, AuthError> {
        let client = state.deref().oauth_client::<P>()?;
        client.authorize()
    }

    /// GET /auth/<provider>/callback
    pub async fn handle_callback(
        state: State<AuthState>,
        cookies: CookieJar,
        Query(query): Query<AuthRequest>,
    ) -> Result<impl IntoResponse, AuthError> {
        let client = state.deref().oauth_client::<P>()?;

        let verified = client.verify(cookies, query).await?;
        let pid = &verified.account_id;

        let user = match state.db.get_user(pid).await? {
            Some(v) => v,
            _ => {
                let _ = state
                    .db
                    .create_user(pid, &verified.username, &verified.email)
                    .await?;
                let Some(v) = state.db.get_user(pid).await? else {
                    return Err(anyhow::anyhow!("impossible"))?;
                };
                v
            },
        };

        // Wow this is obnoxious...  axum_extras::cookie needs a `Duration`.
        // But you would be wrong if you thought that meant
        // `std::time::Duration`.  Oh no, there's another dependency you
        // have to add called `time` just to get `time::Duration`.  This
        // is very different than `std::time::Duration`, used
        // literally everywhere else, and totally incompatible.
        let created_at = Utc::now();
        let exp = i128::from(created_at.timestamp_millis())
            + SESSION_DURATION.whole_milliseconds();
        let expires_at = exp
            .try_into()
            .ok()
            .and_then(DateTime::from_timestamp_millis)
            .context("invalid expiry")?;

        let session_id = state
            .db
            .new_user_session(user.user_id, created_at, expires_at)
            .await?;

        let session = UserSession {
            session_id,
            user_id: user.user_id,
            created_at,
            expires_at,
        };

        let cookies =
            AuthCookies::new_cleared().set_session(session).into_inner();

        Ok((cookies, Redirect::to("/")))
    }
}

/// `UserSession` is created from a verified user and returned in the redirect
/// back to the start page.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct UserSession {
    pub session_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}
