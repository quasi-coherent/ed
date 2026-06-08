use axum::response::Redirect;
use axum_extra::extract::CookieJar;
use oauth2::TokenResponse as _;
use oauth2::basic::BasicClient;
use secrecy::{ExposeSecret as _, SecretString};
use serde::Deserialize;

use crate::AuthError;
use crate::handler::AuthCookies;
use crate::provider::{Provider, VerifiedUser};

/// Login request callback.
#[derive(Clone, Debug, Deserialize)]
pub struct AuthRequest {
    pub code: String,
    pub state: String,
}

/// Client of the authorization server `P`.
#[derive(Clone)]
pub struct OAuthClient<P> {
    provider: P,
    client_id: oauth2::ClientId,
    client_secret: oauth2::ClientSecret,
    redirect_url: oauth2::RedirectUrl,
}

impl<P: Provider> OAuthClient<P> {
    pub fn new(
        id: &SecretString,
        secret: &SecretString,
        redirect_url: url::Url,
        provider: P,
    ) -> Self {
        let client_id = oauth2::ClientId::new(id.expose_secret().to_string());
        let client_secret =
            oauth2::ClientSecret::new(secret.expose_secret().to_string());
        let redirect_url = oauth2::RedirectUrl::from_url(redirect_url);
        Self { provider, client_id, client_secret, redirect_url }
    }

    pub fn authorize(&self) -> Result<(CookieJar, Redirect), AuthError> {
        // Configure the request to send to the authorization server.
        // This is the full authorize URL.
        let client = BasicClient::new(self.client_id.clone())
            .set_client_secret(self.client_secret.clone())
            .set_redirect_uri(self.redirect_url.clone())
            .set_auth_uri(self.provider.auth_url()?)
            .set_token_uri(self.provider.token_url()?);

        let (challenge, verifier) =
            oauth2::PkceCodeChallenge::new_random_sha256();

        let request = client.authorize_url(oauth2::CsrfToken::new_random);
        let (auth_url, csrf_state) = self
            .provider
            .scope()
            .iter()
            .fold(request, |req, scope| req.add_scope(scope.clone()))
            .set_pkce_challenge(challenge)
            .url();

        // Set CSRF state and PKCE code verifier cookies, they are short lived.
        let cookies = AuthCookies::default()
            .set_state(csrf_state)
            .set_verifier(verifier)
            .into_inner();

        Ok((cookies, Redirect::to(auth_url.as_str())))
    }

    pub async fn verify(
        &self,
        cookies: CookieJar,
        AuthRequest { code, state }: AuthRequest,
    ) -> Result<VerifiedUser, AuthError> {
        // Check that the state we stored is still present and unchanged.
        let cookies = AuthCookies::new(cookies);
        if !cookies.is_valid(&state) {
            return Err(AuthError::InvalidState);
        }

        let client = BasicClient::new(self.client_id.clone())
            .set_client_secret(self.client_secret.clone())
            .set_redirect_uri(self.redirect_url.clone())
            .set_auth_uri(self.provider.auth_url()?)
            .set_token_uri(self.provider.token_url()?);

        let http_client = oauth2::reqwest::ClientBuilder::new()
            // Following redirects opens the client up to SSRF vulnerabilities.
            .redirect(oauth2::reqwest::redirect::Policy::none())
            .build()
            .map_err(AuthError::internal)?;

        let code = oauth2::AuthorizationCode::new(code);
        let verifier = cookies.try_get_verifier()?;

        // Redeem code, get access token from the authorization server.
        let token = client
            .exchange_code(code)
            .set_pkce_verifier(verifier)
            .request_async(&http_client)
            .await
            .map_err(AuthError::exchange)?;

        // Token introspection establishes that the user is active and returns
        // the metadata we need to create a legitimate session for the user.
        let introspect = self.provider.introspect_url()?;
        let userinfo = reqwest::Client::new()
            .get(introspect.url().to_owned())
            .bearer_auth(token.access_token().secret())
            .send()
            .await
            .map_err(AuthError::userinfo)?
            .json::<P::Info>()
            .await?;

        self.provider.verified_user(userinfo)
    }
}
