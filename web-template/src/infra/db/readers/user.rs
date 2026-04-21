use async_trait::async_trait;
use sea_orm::{ActiveValue::Set, ConnectionTrait, EntityTrait as _, sea_query::OnConflict};
use std::convert::Infallible;

use crate::{
    application::user::interfaces::UserReader,
    domain::{
        common::errors::ErrKind,
        user::{entity::User, errors::UserNotFound},
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
    async fn get_by_id(&self, id: i64) -> Result<User, ErrKind<UserNotFound>> {
        unimplemented!()
    }
}
