//! Shared domain/application error wrapper.
//!
//! `ErrKind` separates errors the application expects and can model explicitly
//! from unexpected failures such as infrastructure or programming errors. This
//! lets handlers map expected errors to domain-specific responses while still
//! returning a generic internal error for unexpected failures.

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
