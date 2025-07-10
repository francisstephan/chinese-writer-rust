use axum::{Router, routing::get, routing::post};
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tracing::Level;

mod dbase;
mod forms;
mod handlers;

pub struct AppState {
    db: SqlitePool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    let pool = SqlitePool::connect("sqlite://vol/zidian.db").await.unwrap();
    let router: Router<()> = Router::new()
        .route("/", get(handlers::index))
        .route("/size", get(handlers::size))
        .route("/getziform", get(handlers::getziform))
        .route("/zilist", post(handlers::zilist))
        .route("/getpyform", get(handlers::getpyform))
        .route("/pylist", post(handlers::pylist))
        .route("/listdic", get(handlers::listdic))
        .route("/cancel", get(handlers::cancel))
        .route("/zistring", get(handlers::writehanzistring))
        .route("/candidatelist", post(handlers::candidatelist))
        .route("/getparseform", get(handlers::getparseform))
        .route("/stringparse", post(handlers::stringparse))
        .with_state(Arc::new(AppState { db: pool.clone() }))
        .nest_service("/assets", ServeDir::new("./vol/assets"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
