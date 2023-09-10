use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use maud::html;

use crate::{components::Page, routes::not_found, AppState};

pub async fn skills(Path(game_slug): Path<String>, State(state): State<Arc<AppState>>) -> Response {
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

    let skills = sqlx::query!(
        "SELECT skill.name FROM skill JOIN game on game.id = skill.game_id WHERE game.slug = $1;",
        game_slug
    )
    .fetch_all(&state.pool)
    .await;

    Page::new(html! {
        a href={"/games/" (game_slug)} class="text-xl hover:text-violet font-bold" { (game.name) }
        h2 class="text-lg" { "Skills" }
        ul {
            @match skills {
                Ok(skills) => { @for skill in skills { li { (skill.name) } } }
                Err(_) => { p { "No skills..." } }
            }
        }
    })
    .render()
    .into_response()
}
