use std::sync::Arc;

use axum::{
    extract::State,
    response::{IntoResponse, Response},
};
use axum_extra::routing::TypedPath;
use maud::html;
use serde::Deserialize;

use crate::{
    components::Page,
    routes::{games::game, not_found},
    AppState,
};

#[derive(Deserialize, TypedPath)]
#[typed_path("/games/:game_slug/traits")]
pub struct Path {
    game_slug: String,
}

impl Path {
    pub fn new(game_slug: String) -> Self {
        Self { game_slug }
    }
}

pub async fn get(Path { game_slug }: Path, State(state): State<Arc<AppState>>) -> Response {
    let game = sqlx::query!(
        "SELECT id, name, slug, description FROM game WHERE slug = $1;",
        game_slug
    )
    .fetch_one(&state.pool)
    .await;

    if game.is_err() {
        return not_found().await.into_response();
    }

    let game = game.unwrap();

    let traits = sqlx::query!(
        "SELECT trait.name FROM trait JOIN game on game.id = trait.game_id WHERE game.slug = $1;",
        game_slug
    )
    .fetch_all(&state.pool)
    .await;

    Page::new(html! {
        a href=(game::Path::new(game_slug)) class="text-xl font-bold hover:text-violet" { (game.name) }
        h2 class="text-lg" { "Traits" }
        ul {
            @match traits {
                Ok(traits) => { @for r#trait in traits { li { (r#trait.name) } } }
                Err(_) => { p { "No traits..." } }
            }
        }
    })
    .into_response()
}
