use async_trait::async_trait;
use std::convert::Infallible;

use crate::domain::{
    common::errors::ErrKind,
    user::{entity::User, errors::UserNotFound},
};

#[async_trait]
pub trait UserReader: Send + Sync {
    async fn get_by_id(&self, id: i64) -> Result<User, ErrKind<UserNotFound>>;
}

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn upsert(&self, user: User) -> Result<User, ErrKind<Infallible>>;
}
