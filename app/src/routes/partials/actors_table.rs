use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};
use axum_extra::routing::TypedPath;
use maud::html;
use serde::Deserialize;

use crate::AppState;

#[derive(Deserialize, TypedPath)]
#[typed_path("/partials/actors-table/:game_slug/:actor_kind_slug")]
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
    let actors = sqlx::query!(
        "
        SELECT actor.name, actor.slug FROM actor
        JOIN actor_kind ON actor_kind.id = actor.kind_id
        JOIN game ON game.id = actor_kind.game_id
        WHERE game.slug = $1 AND actor_kind.slug = $2;
        ",
        *game_slug,
        *actor_kind_slug
    )
    .fetch_all(&state.pool)
    .await
    .map_or(vec![], |actors| actors);

    html! {
        table class="w-full" {
            thead class="text-xs text-gray-700 uppercase dark:text-gray-400 bg-slate-50 dark:bg-slate-700" {
                tr {
                    th class="p-3 text-center" { input type="checkbox" class="bg-transparent"; }
                    th class="p-3 text-center" { "Name" }
                }
            }
            tbody {
                @for actor in actors.iter() {
                    tr class="bg-white border-b last:border-0 dark:bg-slate-800 dark:border-slate-700" {
                        td class="p-3 text-center" {
                            input type="checkbox" name="slugs" value=(&actor.slug) class="bg-transparent";
                        }
                        td class="p-3 text-center" {
                            a
                                // TODO: avoid clone?
                                href=(crate::routes::games::game::actors::Path::new(game_slug.clone(), Arc::new(actor.slug.clone())))
                                class="hover:text-violet" {
                                (actor.name)
                            }
                        }
                    }
                }
            }
        }
    }
}
