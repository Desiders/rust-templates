use async_trait::async_trait;
use sea_orm::{ConnectionTrait, EntityTrait as _};
use uuid::Uuid;

use crate::{
    application::user::interfaces::UserReader,
    domain::{
        common::errors::ErrKind,
        user::{entities::User, errors::UserByIdNotFound},
    },
    infra::db::models::users,
};

pub struct SeaOrmUserReader<'a, Conn> {
    conn: &'a Conn,
}

impl<'a, Conn> SeaOrmUserReader<'a, Conn> {
    pub const fn new(conn: &'a Conn) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl<Conn: ConnectionTrait> UserReader for SeaOrmUserReader<'_, Conn> {
    async fn get_by_id(&self, id: Uuid) -> Result<User, ErrKind<UserByIdNotFound>> {
        use users::Entity;

        match Entity::find_by_id(id).one(self.conn).await {
            Ok(Some(user)) => Ok(user.into()),
            Ok(None) => Err(ErrKind::Expected(UserByIdNotFound { id })),
            Err(err) => Err(ErrKind::Unexpected(err.into())),
        }
    }
}
