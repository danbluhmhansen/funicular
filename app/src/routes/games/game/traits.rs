use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use maud::html;

use crate::{components::Page, routes::not_found, AppState};

pub async fn traits(Path(game_slug): Path<String>, State(state): State<Arc<AppState>>) -> Response {
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
        a href={"/games/" (game_slug)} class="text-xl hover:text-violet font-bold" { (game.name) }
        h2 class="text-lg" { "Traits" }
        ul {
            @match traits {
                Ok(traits) => { @for t in traits { li { (t.name) } } }
                Err(_) => { p { "No traits..." } }
            }
        }
    })
    .render()
    .into_response()
}
