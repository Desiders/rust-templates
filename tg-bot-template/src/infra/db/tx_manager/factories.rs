use sea_orm::{DatabaseConnection, DatabaseTransaction};

use crate::{
    application::user::interfaces::{UserReader, UserRepo},
    infra::db::{readers::user::SeaOrmUserReader, repos::user::SeaOrmUserRepo},
};

pub trait FactoryReader: Send + Sync {
    fn factory<'a>(&self, conn: &'a DatabaseConnection) -> Box<dyn UserReader + 'a>;
}

pub trait FactoryRepo: Send + Sync {
    fn factory<'a>(&self, conn: &'a DatabaseTransaction) -> Box<dyn UserRepo + 'a>;
}

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

#[derive(Clone)]
pub struct DefaultUserReaderFactory;

impl FactoryReader for DefaultUserReaderFactory {
    fn factory<'a>(&self, conn: &'a DatabaseConnection) -> Box<dyn UserReader + 'a> {
        Box::new(SeaOrmUserReader::new(conn))
    }
}

#[derive(Clone)]
pub struct DefaultUserRepoFactory;

impl FactoryRepo for DefaultUserRepoFactory {
    fn factory<'a>(&self, conn: &'a DatabaseTransaction) -> Box<dyn UserRepo + 'a> {
        Box::new(SeaOrmUserRepo::new(conn))
    }
}
