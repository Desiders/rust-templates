use tracing::{info, instrument};

use crate::{
    application::{common::interactor::Interactor, db::tx_manager::TxManager},
    domain::{
        common::errors::ErrKind,
        user::{entities::User, errors::UserAlreadyExists},
    },
};

pub struct SaveUser {}

pub struct SaveUserInput<'a, TxM: TxManager> {
    pub user: User,
    pub tx_manager: &'a mut TxM,
}

impl<TxM: TxManager> Interactor<SaveUserInput<'_, TxM>> for &SaveUser {
    type Output = User;
    type Err = ErrKind<UserAlreadyExists>;

    #[instrument(skip_all)]
    async fn execute(
        self,
        SaveUserInput { user, tx_manager }: SaveUserInput<'_, TxM>,
    ) -> Result<Self::Output, Self::Err> {
        tx_manager.begin().await?;

        let repo = tx_manager.user_repo()?;
        let user = repo.add(user).await?;
        drop(repo);

        tx_manager.commit().await?;
        info!(%user.id, "User saved");

        Ok(user)
    }
}
