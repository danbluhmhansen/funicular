use std::sync::Arc;

use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use axum_extra::routing::TypedPath;

use crate::AppState;

#[derive(TypedPath)]
#[typed_path("/partials/games-table")]
pub(crate) struct Path;

pub(crate) async fn get(_: Path, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let games = sqlx::query!("SELECT name, slug FROM game;")
        .fetch_all(&state.pool)
        .await
        .map_or(vec![], |games| games);

    Html(markup::new! {
        table["x-data"="{ toggle: false }",class="w-full"] {
            thead[class="text-xs text-gray-700 uppercase dark:text-gray-400 bg-slate-50 dark:bg-slate-700"] {
                tr {
                    th[class="p-3 text-center"] { input[type="checkbox","x-model"="toggle",class="bg-transparent"]; }
                    th[class="p-3 text-center"] { "Name" }
                }
            }
            tbody {
                @for game in games.iter() {
                    tr[class="bg-white border-b last:border-0 dark:bg-slate-800 dark:border-slate-700"] {
                        td[class="p-3 text-center"] {
                            input[type="checkbox",name="slugs",value=&game.slug,":checked"="toggle",class="bg-transparent"];
                        }
                        td[class="p-3 text-center"] {
                            a[
                                // TODO: avoid clone?
                                href=crate::routes::games::game::Path::new(Arc::new(game.slug.clone())).to_string(),
                                class="hover:text-violet"
                            ] {
                                @game.name
                            }
                        }
                    }
                }
            }
        }
    }.to_string())
}
