//! Database layer of ed-api.
//!
//! This exposes a query interface that is sufficient for API requests and a
//! client that implements it.
use secrecy::SecretString;
use std::fmt::{self, Debug, Formatter};
use std::ops::Deref;

mod client;
pub use client::{EdDbClient, EdDbConfig};

mod query;
pub use query::*;

pub mod sql;
pub mod types;

/// A type that has the full query capability.
pub trait EdApiSchemaOwner:
    UserScoped + ReadEdApiSchema + WriteEdApiSchema + 'static
{
}

impl<C> EdApiSchemaOwner for C where
    C: UserScoped + ReadEdApiSchema + WriteEdApiSchema + 'static
{
}

/// `EdApiSchema` encapsulates operations on tables in the `ed_api` schema.
pub struct EdApiSchema(Box<dyn EdApiSchemaOwner>);

impl EdApiSchema {
    pub fn new<C: EdApiSchemaOwner>(inner: C) -> Self {
        Self(Box::new(inner))
    }

    pub async fn try_init(db_url: &SecretString) -> anyhow::Result<Self> {
        let config = EdDbConfig::default();
        let inner = config.try_new_client(db_url).await?;
        Ok(Self::new(inner))
    }
}

impl Debug for EdApiSchema {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("EdApiSchema").finish()
    }
}

impl Deref for EdApiSchema {
    type Target = Box<dyn EdApiSchemaOwner>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// `EdApiSchema` encapsulates operations on tables in the `ed_api` schema.
pub struct EdUserTable(Box<dyn ReadWriteEdUsers>);

impl EdUserTable {
    pub fn new<C: ReadWriteEdUsers + 'static>(inner: C) -> Self {
        Self(Box::new(inner))
    }

    pub async fn try_init(db_url: &SecretString) -> anyhow::Result<Self> {
        let config = EdDbConfig::default();
        let inner = config.try_new_client(db_url).await?;
        Ok(Self::new(inner))
    }
}

impl Debug for EdUserTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("EdUserTable").finish()
    }
}

impl Deref for EdUserTable {
    type Target = Box<dyn ReadWriteEdUsers>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
