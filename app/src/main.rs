use std::{error::Error, sync::Arc, time::Duration};

use axum::{Router, Server};
use axum_extra::routing::RouterExt;
use const_format::formatcp;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tower_livereload::LiveReloadLayer;

mod components;
mod routes;

const BUTTON: &str = "inline-flex items-center py-2 px-4 text-sm font-medium text-center bg-transparent rounded border hover:text-white focus:ring-4 focus:outline-none";
const BUTTON_PRIMARY: &str = formatcp!("{BUTTON} {}", " text-violet-600 border-violet-600 dark:text-violet-300 dark:border-violet-300 hover:bg-violet-500 focus:ring-violet-400 dark:hover:bg-violet-400 dark:focus:ring-violet-500");
// const BUTTON_SUCCESS: &str = formatcp!("{BUTTON} {}", " text-green-600 border-green-600 dark:text-green-300 dark:border-green-300 hover:bg-green-500 focus:ring-green-400 dark:hover:bg-green-400 dark:focus:ring-green-500");
// const BUTTON_WARNING: &str = formatcp!("{BUTTON} {}", " text-amber-600 border-amber-600 dark:text-amber-300 dark:border-amber-300 hover:bg-amber-500 focus:ring-amber-400 dark:hover:bg-amber-400 dark:focus:ring-amber-500");
const BUTTON_ERROR: &str = formatcp!("{BUTTON} {}", " text-red-600 border-red-600 dark:text-red-300 dark:border-red-300 hover:bg-red-500 focus:ring-red-400 dark:hover:bg-red-400 dark:focus:ring-red-500");

pub struct AppState {
    pool: Pool<Postgres>,
}

impl AppState {
    fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let shared_state = Arc::new(AppState::new(
        PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(
                &std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost:28816/funicular".to_string()),
            )
            .await?,
    ));

    let app = Router::new()
        .typed_get(routes::index)
        .typed_get(routes::games::games_get)
        .typed_post(routes::games::games_post)
        .typed_get(routes::partials::games_table)
        .fallback(routes::not_found)
        .with_state(shared_state);

    #[cfg(debug_assertions)]
    let app = app.layer(LiveReloadLayer::new());

    Server::bind(&([0, 0, 0, 0], 1111).into())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
