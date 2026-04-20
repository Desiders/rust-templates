use serde::Serialize;
use std::{
    convert::Infallible,
    error::Error,
    fmt::{self, Display, Formatter},
};

use crate::domain::common::ErrKind;

#[derive(Debug, Serialize)]
pub struct Code {
    pub code: u16,
    pub name: &'static str,
}

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
            ErrKind::Unexpected(_) => errors::UNEXPECTED,
        }
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.code)
    }
}

mod errors {
    use axum::extract::rejection::{JsonRejection, PathRejection, QueryRejection};

    use crate::domain::{code::Code, user::exceptions::UserAlreadyExists};

    #[allow(unused)]
    pub const UNEXPECTED: Code = Code {
        code: 1000,
        name: "Unexpected error",
    };

    macro_rules! code_entity {
        ($($name:ident => ($code:expr, $label:expr)),* $(,)?) => {
            $(
                impl super::IntoCode for $name {
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
}
