//! Top-level facade crate.
pub use ed_db::{self as db, EdApiSchema, EdApiSchemaOwner};

pub mod error;
pub mod handlers;
pub mod prompt;
pub mod router;
pub mod state;

pub use error::AppError;
pub use state::AppState;
