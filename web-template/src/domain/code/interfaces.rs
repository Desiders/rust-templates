//! Error-to-code conversion rules.
//!
//! `IntoCode` is the boundary between typed errors and API error payloads.
//! Domain errors, application wrappers, and extractor rejections can implement
//! this trait to describe which public code/name/message should be returned to
//! clients.
//!
//! The concrete mappings live here so response serializers do not need to know
//! about every domain error type. Expected errors keep their own public code,
//! while unexpected errors are intentionally collapsed into [`Unexpected`].
//!
//! Each registered error also gets a compile-time [`CodeEntity`] impl, which
//! `ErrResponse<E>` uses to pin the error's `code`/`name` in the `OpenAPI`
//! schema. Foreign error types cannot be registered directly (orphan rule);
//! wrap them in local newtypes first (see `crate::api::errors`).

use std::convert::Infallible;

use super::entities::{Code, Unexpected};
use crate::api::errors::{JsonRejection, PathNotFound, PathRejection, QueryRejection};
use crate::domain::{
    common::errors::ErrKind,
    user::errors::{UserAlreadyExists, UserByIdNotFound, UserByUsernameNotFound},
};

/// Compile-time half of the error-code registry.
///
/// `CODE`/`NAME` are what `ErrResponse<E>` pins in the generated `OpenAPI`
/// schema; `IntoCode` produces the same values at runtime, so docs and wire
/// format cannot drift apart.
pub trait CodeEntity {
    const CODE: u16;
    const NAME: &'static str;
    const HAS_VALIDATION: bool = false;
}

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
            ErrKind::Unexpected(_) => Unexpected.into_code(),
        }
    }
}

impl IntoCode for anyhow::Error {
    fn into_code(&self) -> Code {
        Unexpected.into_code()
    }
}

macro_rules! code_entity {
    ($($name:ident => $code:expr),* $(,)?) => {
        $(
            impl CodeEntity for $name {
                const CODE: u16 = $code;
                const NAME: &'static str = stringify!($name);
            }

            impl IntoCode for $name {
                fn into_code(&self) -> Code {
                    Code {
                        code: <$name as CodeEntity>::CODE,
                        name: <$name as CodeEntity>::NAME,
                        message: self.to_string().into(),
                        validation: None,
                    }
                }
            }

            impl utoipa::PartialSchema for $name {
                fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
                    utoipa::openapi::schema::ObjectBuilder::new().into()
                }
            }

            impl utoipa::ToSchema for $name {
                fn name() -> std::borrow::Cow<'static, str> {
                    std::borrow::Cow::Borrowed(<$name as CodeEntity>::NAME)
                }
            }
        )*
    };
}

code_entity! {
    Unexpected => 1000,
    JsonRejection => 1001,
    PathRejection => 1002,
    QueryRejection => 1003,
    UserAlreadyExists => 1004,
    UserByIdNotFound => 1005,
    UserByUsernameNotFound => 1006,
    PathNotFound => 1007,
}
