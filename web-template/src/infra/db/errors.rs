//! Small conversion shortcuts for SeaORM database errors.
//!
//! These `From<DbErr>` implementations let infrastructure code use `?` in
//! places like `transaction.commit().await?` instead of writing repetitive
//! `map_err` calls for every begin/commit/rollback/database operation.

use sea_orm::DbErr;

use crate::{
    application::db::errors::{BeginError, CommitError, RollbackError},
    domain::common::errors::ErrKind,
};

impl From<DbErr> for BeginError {
    fn from(err: DbErr) -> Self {
        Self::new(err.to_string())
    }
}

impl From<DbErr> for CommitError {
    fn from(err: DbErr) -> Self {
        Self::new(err.to_string())
    }
}

impl From<DbErr> for RollbackError {
    fn from(err: DbErr) -> Self {
        Self::new(err.to_string())
    }
}

impl<E> From<DbErr> for ErrKind<E> {
    fn from(err: DbErr) -> Self {
        ErrKind::Unexpected(err.into())
    }
}
