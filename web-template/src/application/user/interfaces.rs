use std::convert::Infallible;

use crate::domain::{
    common::errors::ErrKind,
    user::{entity::User, errors::UserNotFound},
};

pub trait UserReader {
    fn get_by_id(
        &self,
        id: i64,
    ) -> impl Future<Output = Result<User, ErrKind<UserNotFound>>> + Send;
}

pub trait UserRepo {
    fn upsert(&self, user: User) -> impl Future<Output = Result<User, ErrKind<Infallible>>> + Send;
}
