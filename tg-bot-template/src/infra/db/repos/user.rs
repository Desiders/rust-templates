use async_trait::async_trait;
use sea_orm::{ActiveValue::Set, ConnectionTrait, EntityTrait as _, SqlErr};

use crate::{
    application::user::interfaces::UserRepo,
    domain::{
        common::errors::ErrKind,
        user::{entities::User, errors::UserAlreadyExists},
    },
    infra::db::models::users,
};

pub struct SeaOrmUserRepo<'a, Conn> {
    conn: &'a Conn,
}

impl<'a, Conn> SeaOrmUserRepo<'a, Conn> {
    pub const fn new(conn: &'a Conn) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl<Conn: ConnectionTrait> UserRepo for SeaOrmUserRepo<'_, Conn> {
    async fn add(
        &self,
        User {
            tg_id,
            username,
            created_at,
            updated_at,
        }: User,
    ) -> Result<User, ErrKind<UserAlreadyExists>> {
        use users::{ActiveModel, Entity};

        let model = ActiveModel {
            tg_id: Set(tg_id),
            username: Set(username.clone()),
            created_at: Set(created_at),
            updated_at: Set(updated_at),
        };

        Entity::insert(model)
            .exec_with_returning(self.conn)
            .await
            .map(Into::into)
            .map_err(|err| match err.sql_err() {
                Some(SqlErr::UniqueConstraintViolation(_)) => {
                    ErrKind::Expected(UserAlreadyExists { tg_id, username })
                }
                _ => ErrKind::Unexpected(err.into()),
            })
    }
}
