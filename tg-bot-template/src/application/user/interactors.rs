use tracing::{info, instrument};

use crate::{
    application::{common::Interactor, db::tx_manager::TxManager},
    domain::{
        common::errors::ErrKind,
        user::{
            entities::User,
            errors::{UserAlreadyExists, UserByTgIdNotFound},
        },
    },
};

pub struct SaveUser {
    tx_manager: Box<dyn TxManager>,
}

pub struct GetCurrentUser {
    tx_manager: Box<dyn TxManager>,
}

impl SaveUser {
    pub const fn new(tx_manager: Box<dyn TxManager>) -> Self {
        Self { tx_manager }
    }
}

impl GetCurrentUser {
    pub const fn new(tx_manager: Box<dyn TxManager>) -> Self {
        Self { tx_manager }
    }
}

impl Interactor<User> for &SaveUser {
    type Output = User;
    type Err = ErrKind<UserAlreadyExists>;

    #[instrument(skip_all)]
    async fn execute(self, user: User) -> Result<Self::Output, Self::Err> {
        let tx_manager = self.tx_manager.begin().await?;
        let user = User::new(user.tg_id, user.username);

        let user = match {
            let repo = tx_manager.user_repo();
            repo.upsert(user).await
        } {
            Ok(user) => user,
            Err(err) => {
                tx_manager.rollback().await?;
                return Err(err);
            }
        };

        tx_manager.commit().await?;
        info!(tg_id = user.tg_id, "User upserted");

        Ok(user)
    }
}

impl Interactor<i64> for &GetCurrentUser {
    type Output = User;
    type Err = ErrKind<UserByTgIdNotFound>;

    #[instrument(skip_all)]
    async fn execute(self, tg_id: i64) -> Result<Self::Output, Self::Err> {
        let reader = self.tx_manager.user_reader();
        let user = reader.get_by_tg_id(tg_id).await?;
        info!(tg_id = user.tg_id, "Current user received");

        Ok(user)
    }
}
