use sea_orm::{DatabaseConnection, DatabaseTransaction, TransactionTrait as _};
use std::sync::Arc;

use crate::{
    application::{
        db::{
            errors::{BeginError, CommitError, RollbackError, TransactionNotBegin},
            tx_manager::TxManager,
        },
        user::interfaces::{UserReader, UserRepo},
    },
    infra::db::{readers::user::SeaOrmUserReader, repos::user::SeaOrmUserRepo},
};

pub struct SeaOrmTxManager {
    pool: Arc<DatabaseConnection>,
    transaction: Option<DatabaseTransaction>,
}

impl SeaOrmTxManager {
    pub const fn new(pool: Arc<DatabaseConnection>) -> Self {
        Self {
            pool,
            transaction: None,
        }
    }
}

impl TxManager for SeaOrmTxManager {
    async fn begin(&mut self) -> Result<(), BeginError> {
        if self.transaction.is_none() {
            self.transaction = Some(self.pool.begin().await?);
        }
        Ok(())
    }

    async fn commit(&mut self) -> Result<(), CommitError> {
        if let Some(transaction) = self.transaction.take() {
            transaction.commit().await?;
        }
        Ok(())
    }

    async fn rollback(&mut self) -> Result<(), RollbackError> {
        if let Some(transaction) = self.transaction.take() {
            transaction.rollback().await?;
        }
        Ok(())
    }

    fn user_reader(&self) -> impl UserReader + Send {
        SeaOrmUserReader::new(self.pool.as_ref())
    }

    fn user_repo(&self) -> Result<impl UserRepo + Send, TransactionNotBegin> {
        Ok(SeaOrmUserRepo::new(
            self.transaction.as_ref().ok_or(TransactionNotBegin)?,
        ))
    }
}
