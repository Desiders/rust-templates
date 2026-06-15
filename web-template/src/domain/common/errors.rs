//! Shared domain/application error wrapper.
//!
//! `ErrKind` separates errors the application expects and can model explicitly
//! from unexpected failures such as infrastructure or programming errors. This
//! lets controllers map expected errors to domain-specific status codes while
//! still returning a generic internal error for unexpected failures.

use std::convert::Infallible;

/// Classifies a use-case error as expected or unexpected.
///
/// `Expected` contains a concrete domain/application error type, such as
/// `UserAlreadyExists`. `Unexpected` stores an opaque error for failures that
/// should not become part of the public use-case contract.
#[derive(Debug, thiserror::Error)]
#[allow(unused)]
pub enum ErrKind<E> {
    #[error(transparent)]
    Expected(E),
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

impl<E> ErrKind<E> {
    /// Map the expected variant; leaves `Unexpected` untouched.
    #[allow(unused)]
    pub fn map_expected<F>(self, f: impl FnOnce(E) -> F) -> ErrKind<F> {
        match self {
            Self::Expected(err) => ErrKind::Expected(f(err)),
            Self::Unexpected(err) => ErrKind::Unexpected(err),
        }
    }
}

impl ErrKind<Infallible> {
    /// Lift a never-`Expected` error into any `ErrKind<E>`.
    #[allow(unused)]
    pub fn cast<E>(self) -> ErrKind<E> {
        match self {
            Self::Expected(never) => match never {},
            Self::Unexpected(err) => ErrKind::Unexpected(err),
        }
    }
}

/// Generates `impl<E> From<$t> for ErrKind<E>` wrapping each type as `Unexpected`.
#[macro_export]
macro_rules! into_unexpected {
    ($($t:ty),* $(,)?) => {
        $(
            impl<E> From<$t> for $crate::domain::common::errors::ErrKind<E> {
                fn from(err: $t) -> Self {
                    Self::Unexpected(err.into())
                }
            }
        )*
    };
}
