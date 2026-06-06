use ed_api::{SecretStore, AppConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = AppConfig::default();

    let Some(input) = std::env::args().skip(1).next() else {
        return Err(anyhow::anyhow!("missing required input"));
    };
    let store = serde_json::from_str::<SecretStore>(&input)?;

    ed_api::run(config, store).await
}
