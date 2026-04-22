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

pub struct AddUser {
    tx_manager: Box<dyn TxManager>,
}

pub struct GetUserById {
    tx_manager: Box<dyn TxManager>,
}

pub struct GetUserByUsername {
    tx_manager: Box<dyn TxManager>,
}

pub struct GetUsers {
    tx_manager: Box<dyn TxManager>,
}

pub struct DeleteUserById {
    tx_manager: Box<dyn TxManager>,
}

impl AddUser {
    pub const fn new(tx_manager: Box<dyn TxManager>) -> Self {
        Self { tx_manager }
    }
}

impl GetUserById {
    pub const fn new(tx_manager: Box<dyn TxManager>) -> Self {
        Self { tx_manager }
    }
}

impl GetUserByUsername {
    pub const fn new(tx_manager: Box<dyn TxManager>) -> Self {
        Self { tx_manager }
    }
}

impl GetUsers {
    pub const fn new(tx_manager: Box<dyn TxManager>) -> Self {
        Self { tx_manager }
    }
}

impl DeleteUserById {
    pub const fn new(tx_manager: Box<dyn TxManager>) -> Self {
        Self { tx_manager }
    }
}

pub struct GetUsersInput {
    pub pagination: Pagination,
}

impl Interactor<User> for &AddUser {
    type Output = User;
    type Err = ErrKind<UserAlreadyExists>;

    #[instrument(skip_all)]
    async fn execute(self, user: User) -> Result<Self::Output, Self::Err> {
        let tx_manager = self.tx_manager.begin().await?;

        let user = match {
            let repo = tx_manager.user_repo();
            repo.add(user).await
        } {
            Ok(user) => user,
            Err(err) => {
                tx_manager.rollback().await?;
                return Err(err);
            }
        };

        tx_manager.commit().await?;
        info!(%user.id, "User saved");

        Ok(user)
    }
}

impl Interactor<Uuid> for &GetUserById {
    type Output = User;
    type Err = ErrKind<UserByIdNotFound>;

    #[instrument(skip_all)]
    async fn execute(self, id: Uuid) -> Result<Self::Output, Self::Err> {
        let reader = self.tx_manager.user_reader();
        let user = reader.get_by_id(id).await?;
        info!(%user.id, "User received");

        Ok(user)
    }
}

impl Interactor<String> for &GetUserByUsername {
    type Output = User;
    type Err = ErrKind<UserByUsernameNotFound>;

    #[instrument(skip_all)]
    async fn execute(self, username: String) -> Result<Self::Output, Self::Err> {
        let reader = self.tx_manager.user_reader();
        let user = reader.get_by_username(username).await?;
        info!(%user.id, "User received");

        Ok(user)
    }
}

impl Interactor<GetUsersInput> for &GetUsers {
    type Output = Vec<User>;
    type Err = ErrKind<Infallible>;

    #[instrument(skip_all)]
    async fn execute(
        self,
        GetUsersInput { pagination }: GetUsersInput,
    ) -> Result<Self::Output, Self::Err> {
        let reader = self.tx_manager.user_reader();
        let users = reader.get_all(pagination).await?;
        info!(count = %users.len(), "Users received");

        Ok(users)
    }
}

impl Interactor<Uuid> for &DeleteUserById {
    type Output = ();
    type Err = ErrKind<UserByIdNotFound>;

    #[instrument(skip_all)]
    async fn execute(self, id: Uuid) -> Result<Self::Output, Self::Err> {
        let tx_manager = self.tx_manager.begin().await?;

        if let Err(err) = {
            let repo = tx_manager.user_repo();
            repo.delete_by_id(id).await
        } {
            tx_manager.rollback().await?;
            return Err(err);
        }

        tx_manager.commit().await?;
        info!(%id, "User deleted");

        Ok(())
    }
}
