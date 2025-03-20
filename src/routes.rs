use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::{Request, StatusCode},
    Extension, Router,
};

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::{error, instrument, trace, Level};

use crate::{AppState, Cli};

pub fn init(pool: Pool<SqliteConnectionManager>, args: Arc<Cli>) -> Router<()> {
    Router::new()
        .route("/{key}", axum::routing::get(get).put(set).delete(delete))
        .layer(axum::middleware::from_fn(crate::auth::authenticate))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let request_id = uuid::Uuid::new_v4().to_string();

                    tracing::span!(
                        Level::TRACE,
                        "request",
                        %request_id,
                        method = ?request.method(),
                        uri = %request.uri(),
                    )
                })
                .on_request(())
                .on_response(()),
        )
        .layer(
            CorsLayer::new()
                .allow_methods(Any)
                .allow_headers(Any)
                .allow_origin(Any),
        )
        .layer(Extension(args))
        .with_state(pool)
}

#[instrument(skip_all)]
pub async fn set(
    Path(key): Path<String>,
    State(pool): AppState,
    value: String,
) -> Result<(), StatusCode> {
    let conn = pool.get().unwrap();

    let res = conn.execute(
        "INSERT OR REPLACE INTO records (key, value) VALUES (?1, ?2)",
        [&key, &value],
    );

    match res {
        Ok(_) => {
            trace!(key, value, "set record");
            Ok(())
        }

        Err(err) => {
            error!(?err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[instrument(skip_all)]
pub async fn get(Path(key): Path<String>, State(pool): AppState) -> Result<String, StatusCode> {
    let conn = pool.get().unwrap();

    let res = conn.query_row("SELECT value FROM records WHERE key = ?1", [&key], |row| {
        row.get::<_, String>(0)
    });

    match res {
        Ok(value) => {
            trace!(key, value, "got record");
            Ok(value)
        }

        Err(err) => {
            error!(?err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[instrument(skip_all)]
pub async fn delete(Path(key): Path<String>, State(pool): AppState) -> Result<(), StatusCode> {
    let conn = pool.get().unwrap();

    let res = conn.execute("DELETE FROM records WHERE key = ?1", [&key]);

    match res {
        Ok(n) => {
            if n == 0 {
                trace!(key, "record not found");
                Err(StatusCode::NOT_FOUND)
            } else {
                trace!(key, "deleted record");
                Ok(())
            }
        }

        Err(err) => {
            error!(?err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
