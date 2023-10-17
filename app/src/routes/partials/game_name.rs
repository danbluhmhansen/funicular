use std::sync::Arc;

use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use axum_extra::routing::TypedPath;
use serde::Deserialize;

use crate::{components::NotFound, AppState, BUTTON_WARNING};

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
        Html(
            markup::new! {
                div[class="flex flex-row gap-2 justify-center items-center"] {
                    h1[class="text-xl font-bold"] { {&game.name} }
                    a[
                        href={format!("#{}", crate::routes::games::game::Submit::Edit)},
                        class={BUTTON_WARNING}
                    ] { div[class="w-4 h-4 i-tabler-pencil"]{} }
                }
            }
            .to_string(),
        )
    } else {
        Html(NotFound {}.to_string())
    }
}
