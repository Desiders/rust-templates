use super::errors::{BeginError, CommitError, RollbackError, TransactionNotBegin};
use crate::application::user::interfaces::{UserReader, UserRepo};

pub trait TxManager: Send {
    fn begin(&mut self) -> impl Future<Output = Result<(), BeginError>> + Send;
    fn commit(&mut self) -> impl Future<Output = Result<(), CommitError>> + Send;
    fn rollback(&mut self) -> impl Future<Output = Result<(), RollbackError>> + Send;

    fn user_reader(&self) -> impl UserReader + Send;
    fn user_repo(&self) -> Result<impl UserRepo + Send, TransactionNotBegin>;
}
