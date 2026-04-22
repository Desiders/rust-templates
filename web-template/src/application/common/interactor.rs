//! Application use-case abstraction.
//!
//! The template follows a "one handler - one interactor" convention: each HTTP
//! handler delegates its business operation to a dedicated interactor instead of
//! embedding application logic in the controller. This keeps controllers focused
//! on transport concerns such as extractors, status codes, and response mapping.
//!
//! Interactors are created by DI, so infrastructure-facing dependencies such as
//! `TxManager` belong on the interactor itself. `Input` should contain request
//! data for the operation, not dependencies that the container can provide.

/// Executes one application operation for a controller or another caller.
///
/// `Input` should contain the caller-provided data required by the use case.
/// `Output` is the successful result returned to the caller, and `Err` is the
/// domain/application error type that the controller can map into a response.
pub trait Interactor<Input> {
    type Output;
    type Err;

    async fn execute(self, input: Input) -> Result<Self::Output, Self::Err>;
}
