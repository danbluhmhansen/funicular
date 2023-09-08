use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use maud::html;

use crate::{components::Page, AppState};

pub async fn traits(Path(game_slug): Path<String>, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let game = sqlx::query!(
        "SELECT id, name, slug, description FROM game WHERE slug = $1;",
        game_slug
    )
    .fetch_one(&state.pool)
    .await
    .unwrap();

    let traits = sqlx::query!(
        "SELECT trait.name FROM trait JOIN game on game.id = trait.game_id WHERE game.slug = $1;",
        game_slug
    )
    .fetch_all(&state.pool)
    .await
    .unwrap();

    Page::new(html! {
        a href={"/games/" (game_slug)} class="text-xl hover:text-violet font-bold" { (game.name) }
        h2 class="text-lg" { "Traits" }
        ul { @for t in traits { li { (t.name) } } }
    })
    .build()
}
