use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use maud::html;
use serde::Deserialize;

use crate::{components::Page, AppState, BUTTON_ERROR, BUTTON_PRIMARY, BUTTON_SUCCESS, DIALOG};

pub mod actor;

#[derive(Deserialize)]
pub struct ActorsPath {
    pub game_slug: String,
    pub actor_kind_slug: String,
}

pub async fn actors(
    Path(ActorsPath {
        game_slug,
        actor_kind_slug,
    }): Path<ActorsPath>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let game = sqlx::query!(
        "SELECT id, name, slug, description FROM game WHERE slug = $1;",
        game_slug
    )
    .fetch_one(&state.pool)
    .await
    .unwrap();

    let actor_kind = sqlx::query!("SELECT id, name FROM actor_kind WHERE slug = $1", actor_kind_slug)
        .fetch_one(&state.pool)
        .await
        .unwrap();

    let actors = sqlx::query!(
        r#"
            SELECT actor.name, actor.slug
            FROM actor
            JOIN actor_kind ON actor_kind.id = actor.kind_id
            WHERE actor_kind.slug = $1;
        "#,
        actor_kind_slug
    )
    .fetch_all(&state.pool)
    .await;

    Page::new(html! {
        a href={"/games/" (game_slug)} class="text-xl hover:text-violet-500 font-bold" { (game.name) }
        form method="post" enctype="multipart/form-data" class="flex flex-col gap-4 justify-center items-center" {
            input type="hidden" name="kind_id" value=(actor_kind.id);
            div class="overflow-x-auto relative shadow-md rounded w-96" {
                table class="w-full" {
                    caption class="p-3 space-x-2 bg-white dark:bg-slate-800" {
                        a href="#add" class=(BUTTON_PRIMARY) { span class="w-4 h-4 i-tabler-plus"; }
                        button type="submit" name="submit" value="remove" class=(BUTTON_ERROR) {
                            span class="w-4 h-4 i-tabler-trash";
                        }
                    }
                    thead class="text-xs text-gray-700 uppercase dark:text-gray-400 bg-slate-50 dark:bg-slate-700" {
                        tr {
                            th class="p-3 text-center" {
                                input type="checkbox" name="slugs_all" value="true" class="bg-transparent";
                            }
                            th class="py-3 px-6 text-left" { "Name" }
                        }
                    }
                    tbody {
                        @match actors {
                            Ok(actors) => {
                                @for actor in actors {
                                    tr class="bg-white border-b last:border-0 dark:bg-slate-800 dark:border-slate-700" {
                                        td class="p-3 text-center" {
                                            input
                                                type="checkbox"
                                                name="slugs"
                                                value=(actor.slug)
                                                class="bg-transparent";
                                        }
                                        td class="py-3 px-6" {
                                            a
                                                href={"/games/" (game_slug) "/actors/" (actor_kind_slug) "/" (actor.slug)}
                                                class="hover:text-violet-500" {
                                                (actor.name)
                                            }
                                        }
                                    }
                                }
                            },
                            Err(_) => {
                                p { "No games..." }
                            }
                        }
                    }
                }
            }
        }
    })
    .pre(html! {
        dialog id="add" class=(DIALOG) {
            div class="flex z-10 flex-col gap-4 p-4 max-w-sm rounded border dark:text-white dark:bg-slate-900" {
                h2 class="text-xl" { "Add " (actor_kind.name) }
                form method="post" enctype="multipart/form-data" class="flex flex-col gap-4 justify-center" {
                    input type="hidden" name="kind_id" value=(actor_kind.id);
                    input
                        type="text"
                        name="name"
                        placeholder="Name"
                        required
                        autofocus
                        class="bg-transparent rounded invalid:border-red";
                    textarea
                        name="description"
                        placeholder="Description"
                        class="rounded invalid:border-red dark:bg-slate-900" {}
                    div class="flex justify-between" {
                        button type="submit" name="submit" value="add" class=(BUTTON_SUCCESS) {
                            span class="w-4 h-4 i-tabler-check";
                        }
                        a href="#!" class=(BUTTON_PRIMARY) { span class="w-4 h-4 i-tabler-x"; }
                    }
                }
            }
            a href="#!" class="fixed inset-0" {}
        }
    })
    .build()
}
