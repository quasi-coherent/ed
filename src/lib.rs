use anyhow::Context as _;
use axum::Router;
use secrecy::SecretString;
use serde::Deserialize;
use std::net::SocketAddr;
use std::path::Path;
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
    listener_port: u16,
    asset_dir: PathBuf,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            listener_port: std::env::var("APP_PORT")
                .and_then(str::parse)
                .unwrap_or(15625),
            asset_dir: std::env::var("APP_ASSET_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("frontend/dist")),
        }
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

pub async fn run<P: AsRef<Path>>(
    AppConfig { listener_port, asset_dir }: AppConfig,
    store: SecretStore,
) -> anyhow::Result<()> {
    let app = AppState::try_init(store).await?;

    let index_html = asset_dir.as_ref().join("index.html");
    let spa = ServeDir::new(asset_dir).fallback(ServeFile::new(&index_html));

    let router = ed_axum::server::new(app)
        .fallback_service(spa)
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr)
        .await
        .with_context(|| format!("bind {addr}"))?;

    axum::serve(listener, router).await.context("serving app")
}
