use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};
use axum_extra::routing::TypedPath;
use maud::html;
use serde::Deserialize;

use crate::{components::not_found, routes::games::game::Submit, AppState};

#[derive(Deserialize, TypedPath)]
#[typed_path("/partials/game-form/:game_slug")]
pub(crate) struct Path {
    game_slug: Arc<String>,
}

impl Path {
    pub(crate) fn new(game_slug: Arc<String>) -> Self {
        Self { game_slug }
    }
}

pub(crate) async fn get(Path { game_slug }: Path, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    if let Ok(game) = sqlx::query!("SELECT name, description FROM game WHERE slug = $1;", *game_slug)
        .fetch_one(&state.pool)
        .await
    {
        html! {
            form method="post" class="flex flex-col gap-4 justify-center" {
                input
                    type="text"
                    name="name"
                    placeholder="Name"
                    required
                    autofocus
                    value=(&game.name)
                    class="bg-transparent rounded invalid:border-red";
                textarea
                    name="description"
                    placeholder="Description"
                    value=[&game.description]
                    class="bg-transparent rounded invalid:border-red" {}
                div class="flex justify-between" {
                    button type="submit" name="submit" value=(Submit::Edit) class="btn-primary" {
                        span class="w-4 h-4 i-tabler-check";
                    }
                }
            }
        }
    } else {
        not_found()
    }
}
