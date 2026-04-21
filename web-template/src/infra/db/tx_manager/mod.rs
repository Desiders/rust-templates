pub mod factories;

use async_trait::async_trait;
use sea_orm::{DatabaseConnection, DatabaseTransaction, TransactionTrait as _};
use std::sync::Arc;

use crate::application::{
    db::{
        errors::{BeginError, CommitError, RollbackError, TransactionNotBegin},
        tx_manager::TxManager,
    },
    user::interfaces::{UserReader, UserRepo},
};
use factories::TxManagerFactories;

pub struct SeaOrmTxManager {
    pool: Arc<DatabaseConnection>,
    transaction: Option<DatabaseTransaction>,
    factories: Arc<TxManagerFactories>,
}

impl SeaOrmTxManager {
    pub const fn new(pool: Arc<DatabaseConnection>, factories: Arc<TxManagerFactories>) -> Self {
        Self {
            pool,
            transaction: None,
            factories,
        }
    }
}

#[async_trait]
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

    fn user_reader(&self) -> Box<dyn UserReader + '_> {
        self.factories.user_reader.factory(self.pool.as_ref())
    }

    fn user_repo(&self) -> Result<Box<dyn UserRepo + '_>, TransactionNotBegin> {
        let Some(transaction) = self.transaction.as_ref() else {
            return Err(TransactionNotBegin);
        };
        Ok(self.factories.user_repo.factory(transaction))
    }
}
