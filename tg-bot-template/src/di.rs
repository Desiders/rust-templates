use froodi::async_impl::Container;
use froodi::{fragment, registry};

use crate::config::Config;

#[fragment(configuration(cfg))]
registry! {
    use froodi::{DefaultScope::App, instance};

    scope(App) [
        provide(instance(cfg.bot)),
        provide(instance(cfg.database)),
    ],
}

#[fragment(database)]
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
            debug!("Creating database conn");
            match Database::connect(options).await {
                Ok(conn) => {
                    info!("Database conn created");
                    Ok(conn)
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
}

#[fragment(transactions)]
registry! {
    use froodi::{DefaultScope::{App, Request}, Inject, boxed};

    use crate::application::db::tx_manager::TxManager;
    use crate::infra::db::tx_manager::{
        SeaOrmTxManager,
        factories::{DefaultUserReaderFactory, DefaultUserRepoFactory, TxManagerFactories},
    };

    provide(App, || Ok(TxManagerFactories::new(
        Box::new(DefaultUserReaderFactory), Box::new(DefaultUserRepoFactory),
    ))),
    provide(Request, |Inject(pool), Inject(factories)| async move {
        Ok(boxed!(SeaOrmTxManager::new(pool, factories); TxManager))
    }),
}

#[fragment(interactors)]
registry! {
    use froodi::{DefaultScope::Request, InjectTransient};

    use crate::application::user::interactors::{GetCurrentUser, SaveUser};

    scope(Request) [
        provide(|InjectTransient(tx)| async move { Ok(SaveUser::new(tx)) }),
        provide(|InjectTransient(tx)| async move { Ok(GetCurrentUser::new(tx)) }),
    ],
}

pub(super) fn init(cfg: Config) -> Container {
    Container::new(registry! {
        extend_fragment(configuration!(cfg), database!(), transactions!(), interactors!()),
    })
}
