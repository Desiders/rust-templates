use async_trait::async_trait;

use super::errors::{BeginError, CommitError, RollbackError, TransactionNotBegin};
use crate::application::user::interfaces::{UserReader, UserRepo};

#[async_trait]
pub trait TxManager: Send + Sync + 'static {
    async fn begin(&mut self) -> Result<(), BeginError>;
    async fn commit(&mut self) -> Result<(), CommitError>;
    async fn rollback(&mut self) -> Result<(), RollbackError>;

    fn user_reader(&self) -> Box<dyn UserReader + '_>;
    fn user_repo(&self) -> Result<Box<dyn UserRepo + '_>, TransactionNotBegin>;
}
