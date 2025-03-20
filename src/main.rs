use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;

use clap::Parser;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use tracing::dispatcher::{self, Dispatch};
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

pub type AppState = axum::extract::State<Pool<SqliteConnectionManager>>;

mod auth;
mod db;
mod routes;

#[derive(clap::Parser)]
pub struct Cli {
    /// Path to the sqlite database
    #[arg(env = "RESTY_KV_FILE")]
    file: PathBuf,

    /// Address to listen on
    #[arg(long, default_value = "0.0.0.0", env = "RESTY_KV_HOST")]
    host: String,

    /// Port to listen on
    #[arg(short, long, default_value_t = 3000, env = "RESTY_KV_PORT")]
    port: u16,

    /// If provided, will require authentication by Bearer header
    #[arg(short, long, env = "RESTY_KV_TOKEN")]
    token: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Arc::new(Cli::parse());

    let subscriber =
        Registry::default().with(tracing_logfmt::builder().with_timestamp(false).layer());

    dispatcher::set_global_default(Dispatch::new(subscriber))
        .expect("Global logger has already been set!");

    let pool = db::init(&args).await;
    let router = routes::init(pool, args.clone());

    let listener = tokio::net::TcpListener::bind((args.host.deref(), args.port))
        .await
        .unwrap();

    info!("listening on {addr}", addr = listener.local_addr().unwrap());

    axum::serve(listener, router).await.unwrap();
}
