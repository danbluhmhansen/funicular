use std::sync::Arc;

use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use axum_extra::routing::TypedPath;
use serde::Deserialize;

use crate::{routes, AppState};

#[derive(Deserialize, TypedPath)]
#[typed_path("/partials/traits-table/:game_slug")]
pub(crate) struct Path {
    game_slug: Arc<String>,
}

impl Path {
    pub(crate) fn new(game_slug: Arc<String>) -> Self {
        Self { game_slug }
    }
}

pub(crate) async fn get(Path { game_slug }: Path, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let traits = sqlx::query!(
        "
        SELECT trait.name, trait.slug FROM trait
        JOIN game ON game.id = trait.game_id
        WHERE game.slug = $1;
        ",
        *game_slug
    )
    .fetch_all(&state.pool)
    .await
    .map_or(vec![], |traits| traits);

    Html(markup::new! {
        table["x-data"="{ toggle: false }",class="w-full"] {
            thead[class="text-xs text-gray-700 uppercase dark:text-gray-400 bg-slate-50 dark:bg-slate-700"] {
                tr {
                    th[class="p-3 text-center"] { input[type="checkbox","x-model"="toggle",class="bg-transparent"]; }
                    th[class="p-3 text-center"] { "Name" }
                }
            }
            tbody {
                @for r#trait in traits.iter() {
                    tr[class="bg-white border-b last:border-0 dark:bg-slate-800 dark:border-slate-700"] {
                        td[class="p-3 text-center"] {
                            input[
                                type="checkbox",
                                name="slugs",
                                value=&r#trait.slug,
                                ":checked"="toggle",
                                class="bg-transparent"
                            ];
                        }
                        td[class="p-3 text-center"] {
                            @r#trait.name
                        }
                    }
                }
            }
        }
    }.to_string())
}
