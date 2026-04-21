//! Database errors exposed by the application layer.
//!
//! Application traits should not mention SeaORM, SQLx, or any other concrete
//! persistence library. Keeping these errors here preserves that boundary:
//! interactors can depend on stable application error types, while infrastructure
//! code is responsible for converting SeaORM/SQLx errors into them.
//!
//! This also keeps repository and transaction-manager interfaces reusable if the
//! storage backend changes later.

use crate::domain::common::errors::ErrKind;

#[derive(Debug, thiserror::Error)]
#[error("Begin transaction error: {message}")]
pub struct BeginError {
    message: String,
}

impl BeginError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl<E> From<BeginError> for ErrKind<E> {
    fn from(err: BeginError) -> Self {
        ErrKind::Unexpected(err.into())
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Commit transaction error: {message}")]
pub struct CommitError {
    message: String,
}

impl CommitError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl<E> From<CommitError> for ErrKind<E> {
    fn from(err: CommitError) -> Self {
        ErrKind::Unexpected(err.into())
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Rollback transaction error: {message}")]
pub struct RollbackError {
    message: String,
}

impl RollbackError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl<E> From<RollbackError> for ErrKind<E> {
    fn from(err: RollbackError) -> Self {
        ErrKind::Unexpected(err.into())
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Transaction not begin")]
pub struct TransactionNotBegin;

impl<E> From<TransactionNotBegin> for ErrKind<E> {
    fn from(err: TransactionNotBegin) -> Self {
        ErrKind::Unexpected(err.into())
    }
}
