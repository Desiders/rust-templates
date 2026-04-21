use tracing::{info, instrument};

use crate::{
    application::{common::interactor::Interactor, db::tx_manager::TxManager},
    domain::{
        common::errors::ErrKind,
        user::{
            entities::User,
            errors::{UserAlreadyExists, UserByIdNotFound},
        },
    },
};
use uuid::Uuid;

pub struct SaveUser {}
pub struct GetUserById {}

pub struct SaveUserInput<'a> {
    pub user: User,
    pub tx_manager: &'a mut dyn TxManager,
}

pub struct GetUserByIdInput<'a> {
    pub id: Uuid,
    pub tx_manager: &'a mut dyn TxManager,
}

impl Interactor<SaveUserInput<'_>> for &SaveUser {
    type Output = User;
    type Err = ErrKind<UserAlreadyExists>;

    #[instrument(skip_all)]
    async fn execute(
        self,
        SaveUserInput { user, tx_manager }: SaveUserInput<'_>,
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

impl Interactor<GetUserByIdInput<'_>> for &GetUserById {
    type Output = User;
    type Err = ErrKind<UserByIdNotFound>;

    #[instrument(skip_all)]
    async fn execute(
        self,
        GetUserByIdInput { id, tx_manager }: GetUserByIdInput<'_>,
    ) -> Result<Self::Output, Self::Err> {
        let reader = tx_manager.user_reader();
        let user = reader.get_by_id(id).await?;
        info!(%user.id, "User received");

        Ok(user)
    }
}
