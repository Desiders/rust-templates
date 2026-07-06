//! Shared API response structures used by controllers.
//!
//! Controllers return `Resp` so successful values and domain/application errors
//! are serialized with one consistent JSON shape.

mod serializer;

use axum::response::Response;
use serde::Serialize;
use serializer::{JsonSerializer, Serializer};
use std::{borrow::Cow, marker::PhantomData};
use utoipa::{
    ToSchema,
    openapi::{
        RefOr, Schema,
        schema::{Object, ObjectBuilder, Type},
    },
};

use crate::domain::code::{
    entities::Code,
    interfaces::{CodeEntity, IntoCode},
};

/// Controller return type that can represent either a successful payload or an
/// error that can be converted into an API code.
pub enum Resp {}

impl Resp {
    pub fn with_serializer<S: Serializer>() -> RespWithSerializer<S> {
        RespWithSerializer(PhantomData)
    }

    pub fn ok(data: impl Serialize) -> Response {
        Self::with_serializer::<JsonSerializer>().ok(data)
    }

    pub fn err(err: impl IntoCode) -> Response {
        Self::with_serializer::<JsonSerializer>().err(err)
    }
}

pub struct RespWithSerializer<S>(PhantomData<S>);

#[allow(clippy::unused_self)]
impl<S: Serializer> RespWithSerializer<S> {
    pub fn ok(self, data: impl Serialize) -> Response {
        S::ok(OkResponse::from(data))
    }

    pub fn err(self, err: impl IntoCode) -> Response {
        S::err(ErrResponse::from(err))
    }
}

fn success_flag() -> Object {
    ObjectBuilder::new()
        .schema_type(Type::Boolean)
        .enum_values(Some([true]))
        .examples([true])
        .build()
}

/// Successful JSON response structure
#[derive(Serialize, ToSchema)]
pub struct OkResponse<R: ?Sized> {
    #[schema(schema_with = success_flag)]
    is_success: bool,
    result: R,
}

impl<R: Serialize> From<R> for OkResponse<R> {
    fn from(resp: R) -> Self {
        Self {
            result: resp,
            is_success: true,
        }
    }
}

/// Error JSON response structure
#[derive(Serialize)]
pub struct ErrResponse<E> {
    is_success: bool,
    code: u16,
    name: &'static str,
    message: Cow<'static, str>,
    #[serde(skip)]
    _kind: PhantomData<fn() -> E>,
}

impl<E: IntoCode> From<E> for ErrResponse<E> {
    fn from(err: E) -> Self {
        let Code {
            code,
            name,
            message,
        } = err.into_code();
        Self {
            code,
            name,
            message,
            is_success: false,
            _kind: PhantomData,
        }
    }
}

impl<E: CodeEntity> utoipa::__dev::ComposeSchema for ErrResponse<E> {
    fn compose(_generics: Vec<RefOr<Schema>>) -> RefOr<Schema> {
        ObjectBuilder::new()
            .description(Some(format!("Error response: {} ({})", E::NAME, E::CODE)))
            .property(
                "is_success",
                ObjectBuilder::new()
                    .schema_type(Type::Boolean)
                    .enum_values(Some([false]))
                    .examples([false]),
            )
            .required("is_success")
            .property(
                "code",
                ObjectBuilder::new()
                    .schema_type(Type::Integer)
                    .enum_values(Some([E::CODE]))
                    .examples([E::CODE]),
            )
            .required("code")
            .property(
                "name",
                ObjectBuilder::new()
                    .schema_type(Type::String)
                    .enum_values(Some([E::NAME]))
                    .examples([E::NAME]),
            )
            .required("name")
            .property(
                "message",
                ObjectBuilder::new()
                    .schema_type(Type::String)
                    .description(Some("Human-readable error message")),
            )
            .required("message")
            .into()
    }
}

impl<E: CodeEntity> ToSchema for ErrResponse<E> {}
