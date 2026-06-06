use ed_clients::{AnthropicToken, EdServices, OpenAiToken};
use ed_db::EdApiSchema;

use crate::SecretStore;

/// Axum app state for each handler.
#[derive(Clone, Debug)]
pub struct AppState {
    pub db: EdApiSchema,
    pub services: EdServices,
}

impl AppState {
    pub async fn try_init(store: SecretStore) -> anyhow::Result<Self> {
        let anth = AnthropicToken(store.anthropic_api_key);
        let oai = OpenAiToken(store.openai_api_key);
        Ok(Self {
            db: EdApiSchema::try_init(store.db_url).await?,
            services: EdServices::try_init(anth, oai)?,
        })
    }
}

impl AsRef<AppState> for AppState {
    fn as_ref(&self) -> &AppState {
        self
    }
}
