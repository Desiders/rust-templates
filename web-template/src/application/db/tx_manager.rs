//! Transaction manager abstraction used by application interactors.
//!
//! Repository and reader objects borrow a concrete database connection or
//! transaction. Exposing `user_reader` and `user_repo` from `TxManager` keeps
//! those borrows tied to the transaction manager's lifetime (`'_`) instead of
//! forcing controllers or interactors to know how infrastructure connections are
//! stored.
//!
//! Readers are created from the regular database connection, while repositories
//! that mutate data are created from the active transaction. This lets use cases
//! explicitly start, commit, or roll back a transaction and then obtain repos
//! that cannot outlive the manager that owns the underlying connection state.

use async_trait::async_trait;

use super::errors::{BeginError, CommitError, RollbackError, TransactionNotBegin};
use crate::application::user::interfaces::{UserReader, UserRepo};

/// Coordinates transaction lifecycle and creates scoped data access objects.
#[async_trait]
pub trait TxManager: Send + Sync + 'static {
    async fn begin(&mut self) -> Result<(), BeginError>;
    async fn commit(&mut self) -> Result<(), CommitError>;
    async fn rollback(&mut self) -> Result<(), RollbackError>;

    fn user_reader(&self) -> Box<dyn UserReader + '_>;
    fn user_repo(&self) -> Result<Box<dyn UserRepo + '_>, TransactionNotBegin>;
}
