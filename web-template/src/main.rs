use axum::Router;
use std::{net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::Notify};
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _};

use crate::api::controllers;

mod api;
mod application;
mod config;
mod di;
mod domain;
mod infra;
mod utils;

#[tokio::main]
async fn main() {
    let cfg_path = config::get_path();
    let cfg = config::parse_from_fs(&*cfg_path).unwrap();
    let addr: SocketAddr = cfg.serve.addr.parse().unwrap();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::builder().parse_lossy(cfg.logging.dirs.as_ref()))
        .init();

    info!(
        path = %cfg_path,
        log_filter = %cfg.logging.dirs,
        "Loaded config",
    );

    let cfg_registry = di::cfg_registry(cfg);
    let db_registry = di::db_registry(cfg_registry);
    let tx_manager_registry = di::tx_manager_registry(db_registry);
    let interactors_registry = di::interactors_registry(tx_manager_registry);
    let container = di::init(interactors_registry);

    let router = controllers::router();
    let router = froodi::axum::setup_async_default(router, container.clone());

    let shutdown = Arc::new(Notify::new());
    let _ = tokio::join!(
        tokio::spawn(run_server(router, addr, shutdown.clone())),
        tokio::spawn(utils::shutdown::handle(shutdown))
    );

    container.close().await;
    info!("Container closed");
}

async fn run_server(app: Router, addr: SocketAddr, shutdown: Arc<Notify>) {
    info!(%addr, "Run application");
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(async move { shutdown.notified().await })
        .await
        .unwrap();
}
