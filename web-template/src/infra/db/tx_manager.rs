pub mod factories;

use bon::Builder;
use sea_orm::{DatabaseConnection, DatabaseTransaction, TransactionTrait as _};
use std::sync::Arc;

use crate::application::{
    db::{
        errors::{BeginError, CommitError, RollbackError, TransactionNotBegin},
        tx_manager::TxManager,
    },
    user::interfaces::{UserReader, UserRepo},
};
use factories::{FactoryReader, FactoryRepo};

#[derive(Builder)]
pub struct SeaOrmTxManager<UReader, URepo> {
    #[builder(start_fn)]
    pool: Arc<DatabaseConnection>,
    #[builder(skip)]
    transaction: Option<DatabaseTransaction>,
    user_reader_factory: UReader,
    user_repo_factory: URepo,
}

impl<UReader, URepo> TxManager for SeaOrmTxManager<UReader, URepo>
where
    UReader: for<'a> FactoryReader<Res<'a>: UserReader>,
    URepo: for<'a> FactoryRepo<Res<'a>: UserRepo>,
{
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
        self.user_reader_factory.factory(self.pool.as_ref())
    }

    fn user_repo(&self) -> Result<impl UserRepo + Send, TransactionNotBegin> {
        let Some(transaction) = self.transaction.as_ref() else {
            return Err(TransactionNotBegin);
        };
        Ok(self.user_repo_factory.factory(transaction))
    }
}
