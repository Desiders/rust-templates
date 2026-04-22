//! Error-to-code conversion rules.
//!
//! `IntoCode` is the boundary between typed errors and API error payloads.
//! Domain errors, application wrappers, and extractor rejections can implement
//! this trait to describe which public code/name/message should be returned to
//! clients.
//!
//! The concrete mappings live here so response serializers do not need to know
//! about every domain error type. Expected errors keep their own public code,
//! while unexpected errors are intentionally collapsed into `UNEXPECTED`.

use axum::extract::rejection::{JsonRejection, PathRejection, QueryRejection};
use std::convert::Infallible;

use super::{constants::UNEXPECTED, entities::Code};
use crate::domain::{
    common::errors::ErrKind,
    user::errors::{UserAlreadyExists, UserByIdNotFound, UserByUsernameNotFound},
};

pub trait IntoCode {
    #[allow(clippy::wrong_self_convention)]
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

/// Implements `IntoCode` for errors whose public name matches the Rust type.
///
/// The macro keeps code mappings compact while preserving one explicit numeric
/// code per error type. The response name is generated with `stringify!`, and
/// the message still comes from the error's `Display` implementation.
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
    UserByUsernameNotFound => 1006,
}
