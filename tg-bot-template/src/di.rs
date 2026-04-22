use froodi::{
    DefaultScope::{App, Request},
    Inject, InjectTransient, InstantiateErrorKind, Registry,
    async_impl::{Container, RegistryWithSync},
    async_registry, boxed, instance, registry,
};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::sync::Arc;
use tracing::{error, info};

use crate::{
    application::{db::tx_manager::TxManager, user},
    config::{self, Config},
    infra::db::tx_manager::{
        SeaOrmTxManager,
        factories::{DefaultUserReaderFactory, DefaultUserRepoFactory, TxManagerFactories},
    },
};

pub(super) fn cfg_registry(cfg: Config) -> Registry {
    registry! {
        scope(App) [
            provide(instance(cfg.bot)),
            provide(instance(cfg.database)),
        ]
    }
}

pub(super) fn db_registry(cfg: Registry) -> RegistryWithSync {
    async_registry! {
        provide(
            App,
            |Inject(cfg): Inject<config::Database>| async move {
                let mut options = ConnectOptions::new(cfg.get_postgres_url());
                options.sqlx_logging(false);

                match Database::connect(options).await {
                    Ok(database_conn) => {
                        info!("Database conn created");
                        Ok(database_conn)
                    }
                    Err(err) => {
                        error!(%err, "Create database conn error");
                        Err(InstantiateErrorKind::Custom(err.into()))
                    }
                }
            },
            finalizer = |conn: Arc<DatabaseConnection>| async move {
                match conn.close_by_ref().await {
                    Ok(()) => info!("Database conn closed"),
                    Err(err) => error!(%err, "Close database conn error"),
                }
            },
        ),
        extend(cfg),
    }
}

pub(super) fn tx_manager_registry(db: RegistryWithSync) -> RegistryWithSync {
    let factories = registry! {
        scope(App) [
            provide(|| {
                let user_reader = Box::new(DefaultUserReaderFactory);
                let user_repo = Box::new(DefaultUserRepoFactory);
                Ok(TxManagerFactories::new(user_reader, user_repo))
            }),
        ]
    };

    async_registry! {
        provide(
            Request,
            |Inject(pool): Inject<DatabaseConnection>,
             Inject(factories): Inject<TxManagerFactories>| async move {
                Ok(boxed!(SeaOrmTxManager::new(pool, factories); TxManager))
            }
        ),
        extend(db, factories),
    }
}

pub(super) fn interactors_registry(tx_manager: RegistryWithSync) -> RegistryWithSync {
    async_registry! {
        scope(Request) [
            provide(
                |InjectTransient(tx_manager): InjectTransient<Box<dyn TxManager>>| async move {
                    Ok(user::interactors::CreateUser::new(tx_manager))
                }
            ),
            provide(
                |InjectTransient(tx_manager): InjectTransient<Box<dyn TxManager>>| async move {
                    Ok(user::interactors::GetCurrentUser::new(tx_manager))
                }
            ),
        ],
        extend(tx_manager),
    }
}

pub(super) fn init(interactors: RegistryWithSync) -> Container {
    let registry = async_registry! {
        extend(interactors),
    };
    Container::new(registry)
}
