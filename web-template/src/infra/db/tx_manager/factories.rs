//! Factory implementations for database readers and repositories.
//!
//! Use a factory when creating a repo/reader needs runtime input, such as the
//! current database connection or transaction. The factory itself is cheap to
//! store in DI, while each call to `factory` creates the concrete data-access
//! object for that specific connection.
//!
//! `TxManagerFactories` is the collection to extend when adding another
//! repository or reader to the template.

use sea_orm::{DatabaseConnection, DatabaseTransaction};

use crate::{
    application::user::interfaces::{UserReader, UserRepo},
    infra::db::{readers::user::SeaOrmUserReader, repos::user::SeaOrmUserRepo},
};

/// Creates reader trait objects from the shared database connection.
pub trait FactoryReader: Send + Sync {
    fn factory<'a>(&self, conn: &'a DatabaseConnection) -> Box<dyn UserReader + 'a>;
}

/// Creates repository trait objects from the active database transaction.
pub trait FactoryRepo: Send + Sync {
    fn factory<'a>(&self, conn: &'a DatabaseTransaction) -> Box<dyn UserRepo + 'a>;
}

/// Factory collection injected into `SeaOrmTxManager`.
///
/// Add new repo/reader factory fields here as the template grows.
pub struct TxManagerFactories {
    pub user_reader: Box<dyn FactoryReader>,
    pub user_repo: Box<dyn FactoryRepo>,
}

impl TxManagerFactories {
    pub const fn new(user_reader: Box<dyn FactoryReader>, user_repo: Box<dyn FactoryRepo>) -> Self {
        Self {
            user_reader,
            user_repo,
        }
    }
}

/// Default factory for the user reader implementation
#[derive(Clone)]
pub struct DefaultUserReaderFactory;

impl FactoryReader for DefaultUserReaderFactory {
    fn factory<'a>(&self, conn: &'a DatabaseConnection) -> Box<dyn UserReader + 'a> {
        Box::new(SeaOrmUserReader::new(conn))
    }
}

/// Default factory for the user repository implementation
#[derive(Clone)]
pub struct DefaultUserRepoFactory;

impl FactoryRepo for DefaultUserRepoFactory {
    fn factory<'a>(&self, conn: &'a DatabaseTransaction) -> Box<dyn UserRepo + 'a> {
        Box::new(SeaOrmUserRepo::new(conn))
    }
}
