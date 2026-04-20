use froodi::{
    DefaultScope::{App, Request},
    Inject, InstantiateErrorKind, Registry,
    async_impl::{Container, RegistryWithSync},
    async_registry, instance, registry,
};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::sync::Arc;
use tracing::{error, info};

use crate::{
    config::{self, Config},
    infra::db::tx_manager::SeaOrmTxManager,
};

pub(super) fn cfg_registry(cfg: Config) -> Registry {
    registry! {
        scope(App) [
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
                    Ok(()) => {
                        info!("Database conn closed");
                    },
                    Err(err) => {
                        error!(%err, "Close database conn error");
                    },
                }
            },
        ),
        extend(cfg),
    }
}

pub(super) fn tx_manager_registry<UReader, URepo>(
    db: RegistryWithSync,
    user_reader_factory: UReader,
    user_repo_factory: URepo,
) -> RegistryWithSync
where
    UReader: Send + Sync + Clone + 'static,
    URepo: Send + Sync + Clone + 'static,
{
    async_registry! {
        provide(
            Request,
            move |Inject(pool): Inject<DatabaseConnection>| {
                let user_reader_factory = user_reader_factory.clone();
                let user_repo_factory = user_repo_factory.clone();
                async move {
                    Ok(SeaOrmTxManager::builder(pool)
                        .user_reader_factory(user_reader_factory)
                        .user_repo_factory(user_repo_factory)
                        .build()
                    )
                }
            },
        ),
        extend(db),
    }
}

pub(super) fn interactors_registry(cfg: Registry) -> Registry {
    registry! {}
}

pub(super) fn init(interactors: Registry, tx_manager: RegistryWithSync) -> Container {
    let registry = async_registry! {
        extend(interactors, tx_manager),
    };
    Container::new(registry)
}
