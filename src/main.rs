use axum::{Router, routing::get, routing::post};
use axum_htmx::HxRequestGuardLayer;
use rand::{Rng, SeedableRng, rngs::StdRng};
use sqlx::{Pool, Sqlite, sqlite::SqlitePool};
use std::sync::{Arc, Mutex};
use tower_http::services::ServeDir;
use tracing::Level;

mod dbase;
mod forms;
mod handlers;

#[derive(Clone)]
pub struct AppState {
    db: SqlitePool,
    rn: Arc<Mutex<StdRng>>,
}
impl AppState {
    fn new(pool: Pool<Sqlite>) -> Self {
        let rng = StdRng::from_os_rng();
        Self {
            db: pool.clone(),
            rn: Arc::new(Mutex::new(rng)),
        }
    }
    fn next(&self) -> f64 {
        let mut rng = self.rn.lock().expect("Mutex poisoned");
        rng.random_range(0.0..1.0)
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    let pool = SqlitePool::connect("sqlite://vol/zidian.db").await.unwrap();

    let ap: AppState = AppState::new(pool);

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
        .route("/askquiz", get(handlers::askquiz))
        .route("/ansquiz/{param}", get(handlers::ansquiz))
        .with_state(Arc::new(ap))
        .nest_service("/assets", ServeDir::new("./vol/assets"));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
