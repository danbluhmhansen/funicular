use std::sync::Arc;

use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use axum_extra::routing::TypedPath;
use serde::Deserialize;
use strum::Display;

use crate::{
    components::{Layout, NotFound},
    routes, AppState,
};

#[derive(Deserialize, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub(crate) enum Submit {
    Edit,
}
#[derive(Deserialize, TypedPath)]
#[typed_path("/games/:game_slug/actors/:actor_kind_slug/:actor_slug")]
pub(crate) struct Path {
    game_slug: Arc<String>,
    actor_kind_slug: Arc<String>,
    actor_slug: Arc<String>,
}

impl Path {
    pub(crate) fn new(game_slug: Arc<String>, actor_kind_slug: Arc<String>, actor_slug: Arc<String>) -> Self {
        Self {
            game_slug,
            actor_kind_slug,
            actor_slug,
        }
    }
}

pub(crate) async fn get(
    Path {
        game_slug,
        actor_kind_slug,
        actor_slug,
    }: Path,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    if let Ok(actor) = sqlx::query!(
        "
        SELECT
            actor.id,
            actor.name,
            actor.description,
            actor.skills,
            game.name AS game_name,
            actor_kind.name AS actor_kind_name
        FROM actor_skill_agg AS actor
        JOIN actor_kind ON actor_Kind.id = actor.kind_id
        JOIN game ON game.id = actor_kind.game_id
        WHERE game.slug = $1 AND actor_kind.slug = $2 AND actor.slug = $3;
        ",
        *game_slug,
        *actor_kind_slug,
        *actor_slug
    )
    .fetch_one(&state.pool)
    .await
    {
        Html(
            Layout {
                content: markup::new! {
                    ol[class="flex flex-row"] {
                        li {
                            a[
                                href=routes::games::game::Path::new(game_slug.clone()).to_string(),
                                class="hover:text-violet-500"
                            ] { @actor.game_name }
                        }
                        li[class="flex flex-row justify-center items-center"] {
                          span[class="i-tabler-chevron-right"];
                        }
                        li {
                            a[
                                href=routes::games::game::actors::Path::new(
                                    game_slug.clone(),
                                    actor_kind_slug.clone()
                                ).to_string(),
                                class="hover:text-violet-500"
                            ] { @actor.actor_kind_name }
                        }
                    }
                    div[class="flex flex-row gap-2 justify-center items-center"] {
                        h1[class="text-xl font-bold"] { @actor.name }
                        a[href=format!("#{}", Submit::Edit),class="btn-warning"] {
                            span[class="w-4 h-4 i-tabler-pencil"];
                        }
                    }
                    @if let Some((keys, vals)) = actor
                        .skills
                        .as_ref()
                        .and_then(|skills| skills.as_object())
                        .map(|skills| (skills.keys(), skills.values())) {
                        table[class="w-full"] {
                            thead[class="text-xs text-gray-700 uppercase dark:text-gray-400 bg-slate-50 dark:bg-slate-700"] {
                                tr { @for key in keys { th[class="p-3 text-center"] { @key } } }
                            }
                            tbody {
                                tr[class="bg-white border-b last:border-0 dark:bg-slate-800 dark:border-slate-700"] {
                                    @for val in vals {
                                        td[class="p-3 text-center"] {
                                            @if let Some(val) = val.as_number() { @val.to_string() }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
            }
            .to_string(),
        )
    } else {
        Html(Layout { content: NotFound {} }.to_string())
    }
}
