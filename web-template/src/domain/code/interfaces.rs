use axum::extract::rejection::{JsonRejection, PathRejection, QueryRejection};
use std::{convert::Infallible, error::Error};

use super::{constants::UNEXPECTED, entity::Code};
use crate::domain::{common::errors::ErrKind, user::errors::UserAlreadyExists};

pub trait IntoCode: Error {
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

macro_rules! code_entity {
        ($($name:ident => ($code:expr, $label:expr)),* $(,)?) => {
            $(
                impl IntoCode for $name {
                    fn into_code(&self) -> Code {
                        Code {
                            code: $code,
                            name: $label,
                        }
                    }
                }
            )*
        };
    }

code_entity! {
    JsonRejection => (1001, "Parse JSON error"),
    PathRejection => (1002, "Path error"),
    QueryRejection => (1003, "Query error"),
}
code_entity! {
    UserAlreadyExists => (1004, "User already exists"),
}
