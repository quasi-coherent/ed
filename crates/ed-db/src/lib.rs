//! Database layer of ed-api.
//!
//! This exposes a query interface that is sufficient for API requests and a
//! client that implements it.  It also houses migrations for constructing the
//! database.
use std::fmt::{self, Debug, Formatter};
use std::ops::Deref;
use std::sync::Arc;

mod client;
pub use client::{EdDbClient, EdDbConfig};

mod query;
pub use query::{ReadEdApiSchema, WriteEdApiSchema};

pub mod types;

/// A type that has the full query capability.
pub trait EdApiSchemaOwner:
    ReadEdApiSchema + WriteEdApiSchema + Send + Sync + 'static
{
}

impl<C> EdApiSchemaOwner for C where
    C: ReadEdApiSchema + WriteEdApiSchema + Send + Sync + 'static
{
}

/// `EdApiSchema` encapsulates operations on tables in the `ed_api` schema.
#[derive(Clone)]
pub struct EdApiSchema(Arc<dyn EdApiSchemaOwner>);

impl EdApiSchema {
    pub fn new<C: EdApiSchemaOwner>(db: C) -> Self {
        Self(Arc::new(db))
    }
}

impl Debug for EdApiSchema {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("EdApiSchemaOwner").finish()
    }
}

impl Deref for EdApiSchema {
    type Target = Arc<dyn EdApiSchemaOwner>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
