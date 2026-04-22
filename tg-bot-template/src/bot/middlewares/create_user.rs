use froodi::async_impl::Container;
use telers::{
    Request,
    errors::{EventErrorKind, MiddlewareError},
    event::EventReturn,
    middlewares::outer::{Middleware, MiddlewareResponse},
};
use tracing::{error, instrument};

use crate::application::{common::Interactor as _, user::interactors::SaveUser};
use crate::domain::user::entities::User;

#[derive(Clone)]
pub struct CreateUser;

impl Middleware for CreateUser {
    #[instrument(skip_all)]
    async fn call(&mut self, request: Request) -> Result<MiddlewareResponse, EventErrorKind> {
        let Some(from) = request.update.from() else {
            return Ok((request, EventReturn::Finish));
        };
        let Some(container) = request.extensions.get::<Container>() else {
            return Ok((request, EventReturn::Finish));
        };

        let save_user = container
            .get_transient::<SaveUser>()
            .await
            .map_err(MiddlewareError::new)?;
        let user = User::new(from.id, from.username.as_ref().map(ToString::to_string));

        if let Err(err) = save_user.execute(user).await {
            error!(%err, "Save user error");
        }

        Ok((request, EventReturn::Finish))
    }
}
