use telers::{
    Bot, Dispatcher, Router,
    enums::MessageType::Text,
    event::telegram::Handler,
    filters::{Command, MessageType},
};
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _};

use crate::bot::{handlers, middlewares};

mod application;
mod bot;
mod config;
mod di;
mod domain;
mod infra;
mod utils;

#[tokio::main]
async fn main() {
    let cfg_path = config::get_path();
    let cfg = config::parse_from_fs(&*cfg_path).unwrap();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::builder().parse_lossy(cfg.logging.dirs.as_ref()))
        .init();

    info!(
        path = %cfg_path,
        log_filter = %cfg.logging.dirs,
        "Loaded config",
    );

    let bot = Bot::new(cfg.bot.token.clone());

    let cfg_registry = di::cfg_registry(cfg);
    let db_registry = di::db_registry(cfg_registry);
    let tx_manager_registry = di::tx_manager_registry(db_registry);
    let interactors_registry = di::interactors_registry(tx_manager_registry);
    let container = di::init(interactors_registry);

    let router = froodi::telers::setup_async_default(Router::new("main"), container.clone())
        .on_update(|observer| observer.register_outer_middleware(middlewares::CreateUser))
        .on_message(|observer| {
            observer.register(
                Handler::new(handlers::start)
                    .filter(MessageType::one(Text))
                    .filter(Command::many(["start", "help"])),
            )
        });

    let dispatcher = Dispatcher::builder()
        .allowed_updates(router.resolve_used_update_types())
        .main_router(router.configure_default())
        .bot(bot)
        .build();

    match dispatcher.run_polling().await {
        Ok(()) => info!("Bot stopped"),
        Err(err) => error!(%err, "Bot stopped"),
    }

    container.close().await;
    info!("Container closed");
}
