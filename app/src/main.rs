use std::{error::Error, sync::Arc, time::Duration};

use axum::{Router, Server};
use axum_extra::routing::RouterExt;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

mod components;
mod routes;

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
        .typed_get(routes::games::get)
        .typed_post(routes::games::post)
        .typed_get(routes::games::game::get)
        .typed_post(routes::games::game::post)
        .typed_get(routes::partials::games_table::get)
        .typed_get(routes::partials::actor_kinds_table::get)
        .fallback_service(ServeDir::new("assets").fallback(axum::routing::get(routes::not_found)))
        .with_state(shared_state);

    #[cfg(debug_assertions)]
    let app = app.layer(LiveReloadLayer::new());

    Server::bind(&([0, 0, 0, 0], 1111).into())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
