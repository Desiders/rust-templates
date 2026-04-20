use sea_orm::{ActiveValue::Set, ConnectionTrait, EntityTrait as _, sea_query::OnConflict};
use std::convert::Infallible;

use crate::{
    application::user::interfaces::UserRepo,
    domain::{common::errors::ErrKind, user::entity::User},
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

impl<Conn: ConnectionTrait> UserRepo for SeaOrmUserRepo<'_, Conn> {
    async fn upsert(
        &self,
        User {
            id,
            username,
            created_at,
            updated_at,
        }: User,
    ) -> Result<User, ErrKind<Infallible>> {
        use users::{
            ActiveModel,
            Column::{Id, UpdatedAt, Username},
            Entity,
        };

        let model = ActiveModel {
            id: Set(id),
            username: Set(username),
            created_at: Set(created_at),
            updated_at: Set(updated_at),
        };

        Entity::insert(model)
            .on_conflict(
                OnConflict::column(Id)
                    .update_columns([Username, UpdatedAt])
                    .to_owned(),
            )
            .exec_with_returning(self.conn)
            .await
            .map(Into::into)
            .map_err(Into::into)
    }
}
