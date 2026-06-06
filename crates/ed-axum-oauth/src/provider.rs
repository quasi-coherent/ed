use serde::Deserialize;

use crate::AuthError;

/// Authorization server.
pub trait Provider: Send + Sync + Default {
    /// String identifier for the provider.
    const ID: &str;

    /// The user info.
    type Info: for<'a> Deserialize<'a>;

    /// The base authorization URL.
    fn auth_url(&self) -> Result<oauth2::AuthUrl, AuthError>;

    /// The token request URL.
    fn token_url(&self) -> Result<oauth2::TokenUrl, AuthError>;

    /// The token introspection URL.
    fn introspect_url(&self) -> Result<oauth2::IntrospectionUrl, AuthError>;

    /// Scopes provided.
    fn scope(&self) -> Vec<oauth2::Scope>;

    /// Verify this user.
    fn verified_user(
        &self,
        userinfo: Self::Info,
    ) -> Result<VerifiedUser, AuthError>;
}

/// A verified user.
///
/// This can have a session created.
#[derive(Debug, Clone)]
pub struct VerifiedUser {
    pub(crate) account_id: String,
    pub(crate) username: String,
    pub(crate) email: String,
}

/// Response from the `userinfo` endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct GoogleProfile {
    sub: String,
    name: String,
    email: Option<String>,
    email_verified: Option<bool>,
}

/// Google Auth Platform as the provider.
#[derive(Debug, Clone)]
pub struct Google {
    auth_url: &'static str,
    token_url: &'static str,
    introspect_url: &'static str,
    scope: Vec<&'static str>,
}

impl Default for Google {
    fn default() -> Self {
        Self {
            auth_url: "https://accounts.google.com/o/oauth2/v2/auth",
            token_url: "https://oauth2.googleapis.com/token",
            introspect_url: "https://www.googleapis.com/oauth2/v3/userinfo",
            // https://developers.google.com/identity/protocols/oauth2/scopes#google-sign-in
            scope: vec!["openid", "email", "profile"],
        }
    }
}

impl Provider for Google {
    type Info = GoogleProfile;

    const ID: &str = "google";

    fn auth_url(&self) -> Result<oauth2::AuthUrl, AuthError> {
        oauth2::AuthUrl::new(self.auth_url.to_string()).map_err(AuthError::from)
    }

    fn token_url(&self) -> Result<oauth2::TokenUrl, AuthError> {
        oauth2::TokenUrl::new(self.token_url.to_string())
            .map_err(AuthError::from)
    }

    fn introspect_url(&self) -> Result<oauth2::IntrospectionUrl, AuthError> {
        oauth2::IntrospectionUrl::new(self.introspect_url.to_string())
            .map_err(AuthError::from)
    }

    fn scope(&self) -> Vec<oauth2::Scope> {
        self.scope.iter().map(|s| oauth2::Scope::new(s.to_string())).collect()
    }

    fn verified_user(
        &self,
        userinfo: Self::Info,
    ) -> Result<VerifiedUser, AuthError> {
        if let Some(email) = userinfo.email
            && userinfo.email_verified.is_some_and(std::convert::identity)
        {
            Ok(VerifiedUser {
                account_id: userinfo.sub,
                username: userinfo.name,
                email,
            })
        } else {
            Err(AuthError::Userinfo("no user email".into()))
        }
    }
}
