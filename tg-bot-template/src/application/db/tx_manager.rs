use async_trait::async_trait;

use super::errors::{BeginError, CommitError, RollbackError};
use crate::application::user::interfaces::{UserReader, UserRepo};

#[async_trait]
pub trait TxManager: Send + Sync {
    async fn begin(&self) -> Result<Box<dyn ActiveTxManager>, BeginError>;

    fn user_reader(&self) -> Box<dyn UserReader + '_>;
}

#[async_trait]
pub trait ActiveTxManager: Send {
    async fn commit(self: Box<Self>) -> Result<(), CommitError>;
    async fn rollback(self: Box<Self>) -> Result<(), RollbackError>;

    fn user_repo(&self) -> Box<dyn UserRepo + '_>;
}
