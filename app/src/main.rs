use std::{error::Error, net::Ipv4Addr, sync::Arc, time::Duration};

use axum::Router;
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
        .typed_get(routes::games::game::actors::get)
        .typed_post(routes::games::game::actors::post)
        .typed_get(routes::games::game::gears::get)
        .typed_post(routes::games::game::gears::post)
        .typed_get(routes::games::game::actors::actor::get)
        .typed_post(routes::games::game::actors::actor::post)
        .typed_get(routes::partials::games_table::get)
        .typed_get(routes::partials::actor_kinds_table::get)
        .typed_get(routes::partials::gear_kinds_table::get)
        .typed_get(routes::partials::skills_table::get)
        .typed_get(routes::partials::traits_table::get)
        .typed_get(routes::partials::actors_table::get)
        .fallback_service(ServeDir::new("assets").fallback(axum::routing::get(routes::not_found)))
        .with_state(shared_state);

    #[cfg(debug_assertions)]
    let app = app.layer(LiveReloadLayer::new());

    axum::serve(
        tokio::net::TcpListener::bind((Ipv4Addr::new(0, 0, 0, 0), 1111)).await?,
        app,
    )
    .await?;

    Ok(())
}
