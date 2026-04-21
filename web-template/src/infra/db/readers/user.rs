use async_trait::async_trait;
use sea_orm::{
    ColumnTrait as _, ConnectionTrait, EntityTrait as _, QueryFilter as _, QueryOrder as _,
    QuerySelect as _,
};
use std::convert::Infallible;
use uuid::Uuid;

use crate::{
    application::{
        common::entities::{Order, Pagination},
        user::interfaces::UserReader,
    },
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

    async fn get_all(&self, pagination: Pagination) -> Result<Vec<User>, ErrKind<Infallible>> {
        use users::{Column::Id, Entity};

        let mut query = Entity::find();

        if let Some(after_id) = pagination.after_id {
            query = match pagination.order {
                Order::Asc => query.filter(Id.gt(after_id)),
                Order::Desc => query.filter(Id.lt(after_id)),
            };
        }

        query = match pagination.order {
            Order::Asc => query.order_by_asc(Id),
            Order::Desc => query.order_by_desc(Id),
        };

        query = query.limit(pagination.limit());

        query
            .all(self.conn)
            .await
            .map(|users| users.into_iter().map(Into::into).collect())
            .map_err(|err| ErrKind::Unexpected(err.into()))
    }
}
