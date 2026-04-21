use axum::extract::rejection::{JsonRejection, PathRejection, QueryRejection};
use std::convert::Infallible;

use super::{constants::UNEXPECTED, entities::Code};
use crate::domain::{
    common::errors::ErrKind,
    user::errors::{UserAlreadyExists, UserByIdNotFound},
};

pub trait IntoCode {
    fn into_code(&self) -> Code;
}

impl IntoCode for Infallible {
    fn into_code(&self) -> Code {
        match *self {}
    }
}

impl<E: IntoCode> IntoCode for ErrKind<E> {
    fn into_code(&self) -> Code {
        match self {
            ErrKind::Expected(err) => err.into_code(),
            ErrKind::Unexpected(_) => UNEXPECTED,
        }
    }
}

impl IntoCode for anyhow::Error {
    fn into_code(&self) -> Code {
        UNEXPECTED
    }
}

macro_rules! code_entity {
        ($($name:ident => $code:expr),* $(,)?) => {
            $(
                impl IntoCode for $name {
                    fn into_code(&self) -> Code {
                        Code {
                            code: $code,
                            name: stringify!($name),
                            message: self.to_string().into(),
                        }
                    }
                }
            )*
        };
    }

code_entity! {
    JsonRejection => 1001,
    PathRejection => 1002,
    QueryRejection => 1003,
}
code_entity! {
    UserAlreadyExists => 1004,
    UserByIdNotFound => 1005,
}
