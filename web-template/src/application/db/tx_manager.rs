//! Transaction manager abstraction used by application interactors.
//!
//! `TxManager` is the stable object injected into handlers. It can create
//! readers from the regular database connection and can start a new transaction,
//! but it does not store active transaction state inside itself.
//!
//! `ActiveTxManager` owns one active transaction. Repositories are created from
//! this active object, so their lifetime is tied to the transaction owner instead
//! of to the injected manager. This keeps transaction use explicit without
//! requiring a mutable "dummy transaction" or shared mutable state in DI.

use async_trait::async_trait;

use super::errors::{BeginError, CommitError, RollbackError};
use crate::application::user::interfaces::{UserReader, UserRepo};

/// Starts transactions and creates readers that do not need an active transaction.
#[async_trait]
pub trait TxManager: Send + Sync {
    async fn begin(&self) -> Result<Box<dyn ActiveTxManager>, BeginError>;

    fn user_reader(&self) -> Box<dyn UserReader + '_>;
}

/// Owns an active transaction and creates repositories scoped to it.
#[async_trait]
pub trait ActiveTxManager: Send {
    async fn commit(self: Box<Self>) -> Result<(), CommitError>;
    async fn rollback(self: Box<Self>) -> Result<(), RollbackError>;

    fn user_repo(&self) -> Box<dyn UserRepo + '_>;
}
