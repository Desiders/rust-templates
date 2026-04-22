use tracing::{info, instrument};

use std::convert::Infallible;

use crate::{
    application::{
        common::{entities::Pagination, interactor::Interactor},
        db::tx_manager::TxManager,
    },
    domain::{
        common::errors::ErrKind,
        user::{
            entities::User,
            errors::{UserAlreadyExists, UserByIdNotFound, UserByUsernameNotFound},
        },
    },
};
use uuid::Uuid;

pub struct AddUser {}
pub struct GetUserById {}
pub struct GetUserByUsername {}
pub struct GetUsers {}
pub struct DeleteUserById {}

pub struct AddUserInput<'a> {
    pub user: User,
    pub tx_manager: &'a mut dyn TxManager,
}

pub struct GetUserByIdInput<'a> {
    pub id: Uuid,
    pub tx_manager: &'a mut dyn TxManager,
}

pub struct GetUserByUsernameInput<'a> {
    pub username: String,
    pub tx_manager: &'a mut dyn TxManager,
}

pub struct GetUsersInput<'a> {
    pub pagination: Pagination,
    pub tx_manager: &'a mut dyn TxManager,
}

pub struct DeleteUserByIdInput<'a> {
    pub id: Uuid,
    pub tx_manager: &'a mut dyn TxManager,
}

impl Interactor<AddUserInput<'_>> for &AddUser {
    type Output = User;
    type Err = ErrKind<UserAlreadyExists>;

    #[instrument(skip_all)]
    async fn execute(
        self,
        AddUserInput { user, tx_manager }: AddUserInput<'_>,
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

impl Interactor<GetUserByUsernameInput<'_>> for &GetUserByUsername {
    type Output = User;
    type Err = ErrKind<UserByUsernameNotFound>;

    #[instrument(skip_all)]
    async fn execute(
        self,
        GetUserByUsernameInput {
            username,
            tx_manager,
        }: GetUserByUsernameInput<'_>,
    ) -> Result<Self::Output, Self::Err> {
        let reader = tx_manager.user_reader();
        let user = reader.get_by_username(username).await?;
        info!(%user.id, "User received");

        Ok(user)
    }
}

impl Interactor<GetUsersInput<'_>> for &GetUsers {
    type Output = Vec<User>;
    type Err = ErrKind<Infallible>;

    #[instrument(skip_all)]
    async fn execute(
        self,
        GetUsersInput {
            pagination,
            tx_manager,
        }: GetUsersInput<'_>,
    ) -> Result<Self::Output, Self::Err> {
        let reader = tx_manager.user_reader();
        let users = reader.get_all(pagination).await?;
        info!(count = %users.len(), "Users received");

        Ok(users)
    }
}

impl Interactor<DeleteUserByIdInput<'_>> for &DeleteUserById {
    type Output = ();
    type Err = ErrKind<UserByIdNotFound>;

    #[instrument(skip_all)]
    async fn execute(
        self,
        DeleteUserByIdInput { id, tx_manager }: DeleteUserByIdInput<'_>,
    ) -> Result<Self::Output, Self::Err> {
        tx_manager.begin().await?;

        let repo = tx_manager.user_repo()?;
        repo.delete_by_id(id).await?;
        drop(repo);

        tx_manager.commit().await?;
        info!(%id, "User deleted");

        Ok(())
    }
}
