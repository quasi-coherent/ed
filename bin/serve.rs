//! Serves the bundled frontend `dist/` and the JSON API.

use std::{net::SocketAddr, path::PathBuf, process::ExitCode};

use anyhow::Context as _;
use axum::Router;
use ed_api::{AppState, EdApiSchema};
use ed_clients::{EdClientService, EdClientServiceImpl, EdHttpClient};
use ed_db::EdDbConfig;
use ed_migratedb::EdApiMigrations;
use tern::Runner;
use tower_http::{services::ServeDir, trace::TraceLayer};

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt::init();

    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("fatal: {e:#}");
            ExitCode::FAILURE
        },
    }
}

/// TODO: this is uuuuuuuuuuuugly...
async fn run() -> anyhow::Result<()> {
    let database_url = require_env("DATABASE_URL")?;
    require_env("OPENAI_API_KEY")?;
    require_env("ANTHROPIC_API_KEY")?;

    let db_client = EdDbConfig::new("ed-serve")
        .try_new_client(&database_url)
        .await
        .context("connecting to database")?;

    let migrations = EdApiMigrations::new(&database_url)
        .await
        .context("init migrations runner")?;
    Runner::new(migrations)
        .run_apply_all(false)
        .await
        .context("applying migrations")?;

    let schema = EdApiSchema::new(db_client);

    let http = reqwest::Client::new();
    let embeddings = EdHttpClient::embeddings_default(&http)
        .context("init OpenAI embeddings client")?;
    let messages = EdHttpClient::messages_default(&http)
        .context("init Anthropic messages client")?;
    let clients =
        EdClientService::new(EdClientServiceImpl::new(embeddings, messages));

    let state = AppState::new(schema, clients);

    let frontend_dir = std::env::var("FRONTEND_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("frontend/dist"));
    let index = frontend_dir.join("index.html");
    let static_files = ServeDir::new(&frontend_dir)
        .fallback(tower_http::services::ServeFile::new(&index));

    let app: Router = ed_axum::server::new(state)
        .fallback_service(static_files)
        .layer(TraceLayer::new_for_http());

    let port: u16 =
        std::env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    tracing::info!(
        %addr,
        frontend_dir = %frontend_dir.display(),
        "ed-serve starting"
    );

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .with_context(|| format!("bind {addr}"))?;
    axum::serve(listener, app).await.context("serve")?;
    Ok(())
}

fn require_env(key: &str) -> anyhow::Result<String> {
    std::env::var(key).with_context(|| format!("missing env var {key}"))
}
