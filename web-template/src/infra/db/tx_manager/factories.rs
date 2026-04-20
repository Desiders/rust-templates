use sea_orm::{DatabaseConnection, DatabaseTransaction};

use crate::infra::db::{readers::user::SeaOrmUserReader, repos::user::SeaOrmUserRepo};

pub trait FactoryReader: Send + Sync {
    type Res<'a>: Send;

    fn factory<'a>(&self, conn: &'a DatabaseConnection) -> Self::Res<'a>;
}

pub trait FactoryRepo: Send + Sync {
    type Res<'a>: Send;

    fn factory<'a>(&self, conn: &'a DatabaseTransaction) -> Self::Res<'a>;
}

#[derive(Clone)]
pub struct DefaultUserReaderFactory;

impl FactoryReader for DefaultUserReaderFactory {
    type Res<'a> = SeaOrmUserReader<'a, DatabaseConnection>;

    fn factory<'a>(&self, conn: &'a DatabaseConnection) -> Self::Res<'a> {
        SeaOrmUserReader::new(conn)
    }
}

#[derive(Clone)]
pub struct DefaultUserRepoFactory;

impl FactoryRepo for DefaultUserRepoFactory {
    type Res<'a> = SeaOrmUserRepo<'a, DatabaseTransaction>;

    fn factory<'a>(&self, conn: &'a DatabaseTransaction) -> Self::Res<'a> {
        SeaOrmUserRepo::new(conn)
    }
}
