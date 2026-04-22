use async_trait::async_trait;
use sea_orm::{ActiveValue::Set, ConnectionTrait, EntityTrait as _, sea_query::OnConflict};

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
    async fn upsert(
        &self,
        User {
            tg_id,
            username,
            created_at,
            updated_at,
        }: User,
    ) -> Result<User, ErrKind<UserAlreadyExists>> {
        use users::{ActiveModel, Column, Entity};

        let model = ActiveModel {
            tg_id: Set(tg_id),
            username: Set(username),
            created_at: Set(created_at),
            updated_at: Set(updated_at),
        };

        Entity::insert(model)
            .on_conflict(
                OnConflict::column(Column::TgId)
                    .update_columns([Column::Username, Column::UpdatedAt])
                    .to_owned(),
            )
            .exec_with_returning(self.conn)
            .await
            .map(Into::into)
            .map_err(Into::into)
    }
}
