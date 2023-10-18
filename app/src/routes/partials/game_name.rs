use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};
use axum_extra::routing::TypedPath;
use maud::html;
use serde::Deserialize;

use crate::{components::not_found, routes::games::game::Submit, AppState};

#[derive(Deserialize, TypedPath)]
#[typed_path("/partials/game-name/:game_slug")]
pub(crate) struct Path {
    game_slug: Arc<String>,
}

impl Path {
    pub(crate) fn new(game_slug: Arc<String>) -> Self {
        Self { game_slug }
    }
}

pub(crate) async fn get(Path { game_slug }: Path, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    if let Ok(game) = sqlx::query!("SELECT name FROM game WHERE slug = $1;", *game_slug)
        .fetch_one(&state.pool)
        .await
    {
        html! {
            div class="flex flex-row gap-2 justify-center items-center" {
                h1 class="text-xl font-bold" { (&game.name) }
                a href={"#" (Submit::Edit)} class="btn-warning" {
                    span class="w-4 h-4 i-tabler-pencil";
                }
            }
        }
    } else {
        not_found()
    }
}
