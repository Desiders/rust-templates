pub mod factories;

use async_trait::async_trait;
use sea_orm::{DatabaseConnection, DatabaseTransaction, TransactionTrait as _};
use std::sync::Arc;

use crate::application::{
    db::{
        errors::{BeginError, CommitError, RollbackError},
        tx_manager::{ActiveTxManager, TxManager},
    },
    user::interfaces::{UserReader, UserRepo},
};
use factories::TxManagerFactories;

pub struct SeaOrmTxManager {
    pool: Arc<DatabaseConnection>,
    factories: Arc<TxManagerFactories>,
}

impl SeaOrmTxManager {
    pub const fn new(pool: Arc<DatabaseConnection>, factories: Arc<TxManagerFactories>) -> Self {
        Self { pool, factories }
    }
}

pub struct SeaOrmActiveTxManager {
    transaction: DatabaseTransaction,
    factories: Arc<TxManagerFactories>,
}

#[async_trait]
impl TxManager for SeaOrmTxManager {
    async fn begin(&self) -> Result<Box<dyn ActiveTxManager>, BeginError> {
        let transaction = self.pool.begin().await?;
        Ok(Box::new(SeaOrmActiveTxManager {
            transaction,
            factories: self.factories.clone(),
        }))
    }

    fn user_reader(&self) -> Box<dyn UserReader + '_> {
        self.factories.user_reader.factory(self.pool.as_ref())
    }
}

#[async_trait]
impl ActiveTxManager for SeaOrmActiveTxManager {
    async fn commit(self: Box<Self>) -> Result<(), CommitError> {
        self.transaction.commit().await?;
        Ok(())
    }

    async fn rollback(self: Box<Self>) -> Result<(), RollbackError> {
        self.transaction.rollback().await?;
        Ok(())
    }

    fn user_repo(&self) -> Box<dyn UserRepo + '_> {
        self.factories.user_repo.factory(&self.transaction)
    }
}
