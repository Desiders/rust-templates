use froodi::async_impl::{Container, TypedContainer, TypedContainerExt as _};
use froodi::{ResolveErrorKind, fragment, registry};
use sea_orm::DatabaseConnection;

use crate::config::Config;

#[fragment(pub configuration(cfg))]
registry! {
    use froodi::{DefaultScope::App, instance};

    provide(App, instance(cfg.database)),
}

#[fragment(pub database)]
registry! {
    use std::sync::Arc;

    use froodi::{DefaultScope::App, Inject, InstantiateErrorKind};
    use sea_orm::{ConnectOptions, Database, DatabaseConnection};
    use tracing::{debug, error, info};

    use crate::config;

    provide(
        App,
        |Inject(cfg): Inject<config::Database>| async move {
            let mut options = ConnectOptions::new(cfg.get_postgres_url());
            options.sqlx_logging(false);
            debug!("Creating database connection pool");
            match Database::connect(options).await {
                Ok(conn) => {
                    info!("Database connection pool created");
                    Ok(conn)
                }
                Err(err) => {
                    error!(%err, "Create database connection error");
                    Err(InstantiateErrorKind::Custom(err.into()))
                }
            }
        },
        finalizer = |conn: Arc<DatabaseConnection>| async move {
            match conn.close_by_ref().await {
                Ok(()) => info!("Database connection pool closed"),
                Err(err) => error!(%err, "Close database connection error"),
            }
        },
    ),
}

#[fragment(pub transactions)]
registry! {
    use froodi::{DefaultScope::App, Inject, boxed};

    use crate::application::db::tx_manager::TxManager;
    use crate::infra::db::tx_manager::{
        SeaOrmTxManager,
        factories::{DefaultUserReaderFactory, DefaultUserRepoFactory, TxManagerFactories},
    };

    scope(App) [
        provide(|| Ok(TxManagerFactories::new(
            Box::new(DefaultUserReaderFactory), Box::new(DefaultUserRepoFactory),
        ))),
        provide(|Inject(pool), Inject(factories)| async move {
            Ok(boxed!(SeaOrmTxManager::new(pool, factories); TxManager))
        }),
    ],
}

#[fragment(pub interactors)]
registry! {
    use froodi::{DefaultScope::Request, Inject};

    use crate::application::user::interactors::{
        AddUser, DeleteUserById, GetUserById, GetUserByUsername, GetUsers,
    };

    scope(Request) [
        provide(|Inject(tx)| async move { Ok(AddUser::new(tx)) }),
        provide(|Inject(tx)| async move { Ok(GetUserById::new(tx)) }),
        provide(|Inject(tx)| async move { Ok(GetUserByUsername::new(tx)) }),
        provide(|Inject(tx)| async move { Ok(GetUsers::new(tx)) }),
        provide(|Inject(tx)| async move { Ok(DeleteUserById::new(tx)) }),
    ],
}

/// Resolve the database before accepting requests, closing resources on failure.
///
/// # Errors
/// Returns an error if the database connection cannot be created.
pub async fn init(cfg: Config) -> Result<Container, ResolveErrorKind> {
    let container = TypedContainer::new(registry! {
        extend_fragment(configuration!(cfg), database!(), transactions!(), interactors!()),
    });
    if let Err(err) = container.get::<DatabaseConnection>().await {
        container.close().await;
        return Err(err);
    }
    Ok(container.into_container())
}
