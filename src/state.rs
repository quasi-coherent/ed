use ed_clients::{AnthropicToken, EdServices, OpenAiToken};
use ed_db::EdApiSchema;
use std::sync::Arc;
use secrecy::SecretString;

/// Axum app state for each handler.
#[derive(Clone, Debug)]
pub struct AppState {
    pub db: Arc<EdApiSchema>,
    pub services: Arc<EdServices>,
}

impl AppState {
    pub async fn try_init(
        anth: AnthropicToken,
        oai: OpenAiToken,
        db_url: &SecretString,
    ) -> anyhow::Result<Self> {
        let db = EdApiSchema::try_init(db_url).await?;
        let svc = EdServices::try_init(anth, oai)?;
        Ok(Self { db: Arc::new(db), services: Arc::new(svc) })
    }
}

impl AsRef<AppState> for AppState {
    fn as_ref(&self) -> &AppState {
        self
    }
}
