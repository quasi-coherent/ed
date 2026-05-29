use ed_clients::EdClientService;
use ed_db::EdApiSchema;

/// Shared application state passed to every handler.
#[derive(Clone, Debug)]
pub struct AppState {
    pub db: EdApiSchema,
    pub clients: EdClientService,
}

impl AppState {
    pub fn new(db: EdApiSchema, clients: EdClientService) -> Self {
        Self { db, clients }
    }
}

impl AsRef<AppState> for AppState {
    fn as_ref(&self) -> &AppState {
        self
    }
}
