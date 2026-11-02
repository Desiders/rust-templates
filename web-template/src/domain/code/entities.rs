//! API error code payload.
//!
//! `Code` is the stable domain representation used to build error responses.
//! It keeps the numeric code, machine-readable name, and human-readable message
//! together before the API layer serializes them into `ErrResponse`.

use serde::Serialize;
use std::{
    borrow::Cow,
    fmt::{self, Display, Formatter},
};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ValidationDetail {
    pub field: String,
    pub reason: String,
}

#[derive(Debug, Serialize)]
pub struct Code {
    #[allow(clippy::struct_field_names)]
    pub code: u16,
    pub name: &'static str,
    pub message: Cow<'static, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation: Option<ValidationDetail>,
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.code)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Unexpected error occurred")]
pub struct Unexpected;
