use std::sync::Arc;

use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use axum_extra::routing::TypedPath;
use serde::Deserialize;

use crate::AppState;

#[derive(Deserialize, TypedPath)]
#[typed_path("/partials/actor-kinds-table/:game_slug")]
pub(crate) struct Path {
    game_slug: Arc<String>,
}

impl Path {
    pub(crate) fn new(game_slug: Arc<String>) -> Self {
        Self { game_slug }
    }
}

pub(crate) async fn get(Path { game_slug }: Path, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let kinds = sqlx::query!(
        "
            SELECT actor_kind.name, actor_kind.slug FROM actor_kind
            JOIN game ON game.id = actor_kind.game_id
            WHERE game.slug = $1;
            ",
        *game_slug
    )
    .fetch_all(&state.pool)
    .await
    .map_or(vec![], |kinds| kinds);
    Html(markup::new! {
        table."w-full" {
            thead."text-xs"."text-gray-700".uppercase."dark:text-gray-400"."bg-slate-50"."dark:bg-slate-700" {
                tr {
                    th."p-3"."text-center" { input."bg-transparent"[type="checkbox"]; }
                    th."p-3"."text-center" { "Name" }
                }
            }
            tbody {
                @for kind in kinds.iter() {
                    tr."bg-white"."border-b"."last:border-0"."dark:bg-slate-800"."dark:border-slate-700" {
                        td."p-3"."text-center" {
                            input."bg-transparent"[type="checkbox",name="slugs",value={&kind.slug}];
                        }
                        td."p-3"."text-center" {
                            a."hover:text-violet"
                                // TODO: avoid clone?
                                [href={crate::routes::games::game::Path::new(Arc::new(kind.slug.clone())).to_string()}] {
                                @kind.name
                            }
                        }
                    }
                }
            }
        }
    }.to_string())
}
