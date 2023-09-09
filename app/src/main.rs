use std::{error::Error, sync::Arc, time::Duration};

use axum::{routing::get, Router};
use const_format::formatcp;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod components;
mod routes;

const STYLE: &str = include_str!("site.css");

const BUTTON: &str = "inline-flex items-center py-2 px-4 text-sm font-medium text-center bg-transparent rounded border hover:text-white focus:ring-4 focus:outline-none";
const BUTTON_PRIMARY: &str = formatcp!("{BUTTON} {}", " text-violet-600 border-violet-600 dark:text-violet-300 dark:border-violet-300 hover:bg-violet-500 focus:ring-violet-400 dark:hover:bg-violet-400 dark:focus:ring-violet-500");
const BUTTON_SUCCESS: &str = formatcp!("{BUTTON} {}", " text-green-600 border-green-600 dark:text-green-300 dark:border-green-300 hover:bg-green-500 focus:ring-green-400 dark:hover:bg-green-400 dark:focus:ring-green-500");
const BUTTON_WARNING: &str = formatcp!("{BUTTON} {}", " text-amber-600 border-amber-600 dark:text-amber-300 dark:border-amber-300 hover:bg-amber-500 focus:ring-amber-400 dark:hover:bg-amber-400 dark:focus:ring-amber-500");
const BUTTON_ERROR: &str = formatcp!("{BUTTON} {}", " text-red-600 border-red-600 dark:text-red-300 dark:border-red-300 hover:bg-red-500 focus:ring-red-400 dark:hover:bg-red-400 dark:focus:ring-red-500");

const DIALOG: &str = "hidden z-10 justify-center items-center w-full h-full target:flex bg-black/50 backdrop-blur-sm";

const CAPTION: &str = "p-3 space-x-2 bg-white dark:bg-slate-800";
const THEAD: &str = "text-xs text-gray-700 uppercase dark:text-gray-400 bg-slate-50 dark:bg-slate-700";
const TR: &str = "bg-white border-b last:border-0 dark:bg-slate-800 dark:border-slate-700";

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
                &std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost:28815/funicular".to_string()),
            )
            .await?,
    ));

    axum::Server::bind(&"0.0.0.0:1111".parse()?)
        .serve(
            Router::new()
                .route("/site.css", get(routes::style))
                .route("/", get(routes::index))
                .route("/games", get(routes::games::games_get).post(routes::games::games_post))
                .route(
                    "/games/:game_slug",
                    get(routes::games::game::game_get).post(routes::games::game::game_post),
                )
                .route(
                    "/games/:game_slug/actors/:actor_kind_slug",
                    get(routes::games::game::actors::actors_get).post(routes::games::game::actors::actors_post),
                )
                .route(
                    "/games/:game_slug/actors/:actor_kind_slug/:actor_slug",
                    get(routes::games::game::actors::actor::actor_get)
                        .post(routes::games::game::actors::actor::actor_post),
                )
                .route("/games/:game_slug/skills", get(routes::games::game::skills::skills))
                .route("/games/:game_slug/traits", get(routes::games::game::traits::traits))
                .fallback(get(routes::not_found))
                .with_state(shared_state)
                .into_make_service(),
        )
        .await?;

    Ok(())
}
