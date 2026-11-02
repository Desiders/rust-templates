use std::net::SocketAddr;
use std::process;

use axum::Router;
use clap::{Parser, Subcommand};
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _};

use web_template::{api::controllers, config, di, openapi_yaml, utils};

const OPENAPI_PATH: &str = "openapi.yaml";

#[derive(Parser)]
#[command(name = "web_template")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Serve the HTTP/REST API.
    Api,
    /// Write the OpenAPI document, then exit.
    OpenapiDump,
    /// Verify the committed OpenAPI document; exit non-zero on drift.
    OpenapiCheck,
}

#[tokio::main]
async fn main() {
    match Cli::parse().command {
        command @ (Command::OpenapiDump | Command::OpenapiCheck) => run_openapi(&command),
        Command::Api => run_api().await,
    }
}

async fn run_api() {
    let cfg_path = config::get_path();
    let cfg = config::parse_from_fs(&*cfg_path)
        .unwrap_or_else(|err| panic!("failed to load config: {err}"));

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::builder().parse_lossy(cfg.logging.dirs.as_ref()))
        .init();

    info!(path = %cfg_path, log_filter = %cfg.logging.dirs, "Loaded config");

    let addr: SocketAddr = cfg.serve.addr.parse().expect("invalid serve.addr");
    let container = di::init(cfg)
        .await
        .expect("initialize application dependencies");
    let router = controllers::router();
    let router = froodi::axum::setup_async_default(router, container.clone());

    let shutdown = CancellationToken::new();
    let _ = tokio::join!(
        tokio::spawn(run_server(router, addr, shutdown.clone())),
        tokio::spawn(utils::shutdown::handle(shutdown))
    );

    container.close().await;
    info!("Container closed");
}

async fn run_server(app: Router, addr: SocketAddr, shutdown: CancellationToken) {
    info!(%addr, "Run application");
    let listener = TcpListener::bind(addr).await.expect("bind listener");
    axum::serve(listener, app)
        .with_graceful_shutdown(async move { shutdown.cancelled().await })
        .await
        .expect("server error");
}

fn run_openapi(command: &Command) {
    let generated = openapi_yaml();
    match command {
        Command::OpenapiDump => {
            std::fs::write(OPENAPI_PATH, generated).expect("write openapi document");
            println!("wrote {OPENAPI_PATH}");
        }
        Command::OpenapiCheck => match std::fs::read_to_string(OPENAPI_PATH) {
            Ok(committed) if committed == generated => println!("{OPENAPI_PATH} is up to date"),
            Ok(_) => {
                eprintln!("{OPENAPI_PATH} is stale — regenerate with `openapi-dump`");
                process::exit(1);
            }
            Err(err) => {
                eprintln!("read {OPENAPI_PATH}: {err} — generate it with `openapi-dump`");
                process::exit(1);
            }
        },
        Command::Api => unreachable!("handled by main"),
    }
}
