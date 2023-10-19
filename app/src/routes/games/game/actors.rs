use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};
use axum_extra::routing::TypedPath;
use maud::html;
use serde::Deserialize;
use strum::Display;

use crate::{
    components::{layout, not_found},
    AppState,
};

#[derive(Deserialize, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub(crate) enum Submit {
    Edit,
    Add,
    Remove,
}

#[derive(Deserialize, TypedPath)]
#[typed_path("/games/:game_slug/actors/:actor_kind_slug")]
pub(crate) struct Path {
    game_slug: Arc<String>,
    actor_kind_slug: Arc<String>,
}

impl Path {
    pub(crate) fn new(game_slug: Arc<String>, actor_kind_slug: Arc<String>) -> Self {
        Self {
            game_slug,
            actor_kind_slug,
        }
    }
}

pub(crate) async fn get(
    Path {
        game_slug,
        actor_kind_slug,
    }: Path,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    if let Ok(actor_kind) = sqlx::query!(
        "
        SELECT actor_kind.id, actor_kind.name, actor_kind.description FROM actor_kind
        JOIN game ON game.id = actor_kind.game_id
        WHERE game.slug = $1 AND actor_kind.slug = $2;
        ",
        *game_slug,
        *actor_kind_slug
    )
    .fetch_one(&state.pool)
    .await
    {
        layout(html! {
            div class="flex flex-row gap-2 justify-center items-center" {
                h1 class="text-xl font-bold" { (&actor_kind.name) }
                a href={"#" (Submit::Edit)} class="btn-warning" {
                    span class="w-4 h-4 i-tabler-pencil";
                }
            }
            div class="overflow-x-auto relative rounded shadow-md" {
                form method="post" {
                    input type="hidden" name="actor_kind_id" value=(&actor_kind.id);
                    div class="flex flex-row gap-2 justify-center p-3 bg-white dark:bg-slate-800" {
                        a href={"#" (Submit::Add)} class="btn-primary" hx-boost="false" {
                            span class="w-4 h-4 i-tabler-plus";
                        }
                        button type="submit" name="submit" value=(Submit::Remove) class="btn-error" {
                            span class="w-4 h-4 i-tabler-trash";
                        }
                    }
                    div
                        hx-get=(crate::routes::partials::actors_table::Path::new(game_slug.clone(), actor_kind_slug.clone()))
                        hx-trigger="revealed"
                        hx-swap="outerHTML" {
                        span class="w-6 h-6 i-svg-spinners-gooey-balls-2";
                    }
                }
            }
        })
    } else {
        layout(not_found())
    }
}
