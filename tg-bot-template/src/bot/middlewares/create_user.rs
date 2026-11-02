use froodi::async_impl::Container;
use telers::{
    Request,
    errors::MiddlewareError,
    middlewares::outer::{Middleware, MiddlewareResult, Next},
};
use tracing::{error, instrument};

use crate::application::{common::Interactor as _, user::interactors::SaveUser};
use crate::domain::user::entities::User;

#[derive(Clone)]
pub struct CreateUser;

impl Middleware for CreateUser {
    #[instrument(skip_all)]
    async fn call(&mut self, request: Request, next: Next) -> MiddlewareResult {
        let Some(from) = request.update.from() else {
            return next(request).await;
        };
        let Some(container) = request.extensions.get::<Container>() else {
            return next(request).await;
        };

        let save_user = match container.get_transient::<SaveUser>().await {
            Ok(interactor) => interactor,
            Err(err) => return Err((MiddlewareError::new(err).into(), request)),
        };
        let user = User::new(from.id, from.username.as_ref().map(ToString::to_string));

        if let Err(err) = save_user.execute(user).await {
            error!(%err, "Save user error");
        }

        next(request).await
    }
}
