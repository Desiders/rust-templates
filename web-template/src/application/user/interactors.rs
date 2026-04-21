use std::convert::Infallible;
use tracing::{info, instrument};

use crate::{
    application::{
        common::interactor::Interactor, db::tx_manager::TxManager, user::interfaces::UserRepo,
    },
    domain::{common::errors::ErrKind, user::entity::User},
};

pub struct SaveUser {}

pub struct SaveUserInput<'a, TxM: TxManager> {
    pub user: User,
    pub tx_manager: &'a mut TxM,
}

impl<'a, TxM: TxManager> Interactor<SaveUserInput<'a, TxM>> for SaveUser {
    type Output = User;
    type Err = ErrKind<Infallible>;

    #[instrument(skip_all)]
    async fn execute(
        self,
        SaveUserInput { user, tx_manager }: SaveUserInput<'a, TxM>,
    ) -> Result<Self::Output, Self::Err> {
        tx_manager.begin().await?;

        let repo = tx_manager.user_repo()?;
        let user = repo.upsert(user).await?;
        drop(repo);

        tx_manager.commit().await?;
        info!(%user.id, "User saved");

        Ok(user)
    }
}
