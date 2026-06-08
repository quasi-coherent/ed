use anyhow::Context as _;
use axum::routing::{Router, get};
use ed_axum_oauth::{AuthHandler, AuthState, providers};
use secrecy::SecretString;
use serde::Deserialize;
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

mod error;
mod handlers;
mod prompt;
mod router;

mod state;
use state::AppState;

pub struct AppConfig {
    port: u16,
    frontend_dir: PathBuf,
    redirect_url: String,
}

impl AppConfig {
    pub fn new() -> anyhow::Result<Self> {
        let port = std::env::var("APP_PORT")
            .as_deref()
            .ok()
            .and_then(|v| str::parse(v).ok())
            .unwrap_or(15625);
        let frontend_dir = std::env::var("APP_FRONTEND_DIR")
            .map(PathBuf::from)
            .context("frontend dir")?;
        let redirect_url = std::env::var("APP_REDIRECT_URL")?;
        Ok(Self { port, frontend_dir, redirect_url })
    }
}

#[derive(Deserialize)]
pub struct SecretStore {
    anthropic_api_key: SecretString,
    openai_api_key: SecretString,
    db_url: SecretString,
    google_client_id: SecretString,
    google_client_secret: SecretString,
}

fn auth_router(state: AuthState) -> axum::Router {
    let google_auth = AuthState::router::<providers::Google>();
    Router::new()
        .route("/auth", get(AuthHandler::handle_get_providers))
        .nest("/google", google_auth)
        .with_state(state)
}

pub async fn run(
    AppConfig { port, frontend_dir, redirect_url }: AppConfig,
    store: SecretStore,
) -> anyhow::Result<()> {
    let idx = frontend_dir.join("index.html");
    let index = ServeFile::new(&idx);
    let fe = ServeDir::new(frontend_dir);

    let auth = AuthState::new(
        redirect_url,
        store.google_client_id,
        store.google_client_secret,
        &store.db_url,
    )
    .await?;
    let auth_router = auth_router(auth);

    let anth = ed_clients::AnthropicToken(store.anthropic_api_key);
    let oai = ed_clients::OpenAiToken(store.openai_api_key);
    let db_url = store.db_url;
    let app = AppState::try_init(anth, oai, &db_url).await?;

    let router = Router::new()
        .route_service("/", fe)
        .fallback_service(index)
        .nest("/v1", auth_router.merge(ed_axum::server::new(app)))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr)
        .await
        .with_context(|| format!("bind {addr}"))?;

    axum::serve(listener, router).await.context("serving app")
}
