use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use maud::html;

use crate::{components::Page, AppState, BUTTON_ERROR, BUTTON_PRIMARY, BUTTON_SUCCESS, BUTTON_WARNING, DIALOG};

pub async fn game(Path(game_slug): Path<String>, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let game = sqlx::query!(
        "SELECT id, name, slug, description FROM game WHERE slug = $1;",
        game_slug
    )
    .fetch_one(&state.pool)
    .await
    .unwrap();

    let actor_kinds = sqlx::query!(
        r#"
            SELECT actor_kind.name, actor_kind.slug
            FROM actor_kind
            JOIN game ON game.id = actor_kind.game_id
            WHERE game.slug = $1
        "#,
        game_slug
    )
    .fetch_all(&state.pool)
    .await
    .unwrap();

    Page::new(html! {
        h1 class="text-xl font-bold" { (game.name) }
        div class="flex flex-row gap-2" {
            a href="#edit" class=(BUTTON_WARNING) { span class="w-4 h-4 i-tabler-pencil"; }
        }
        ul class="flex flex-col gap-4" {
            li class="flex flex-col gap-2" {
                h2 class="text-center" { "Actors" }
                form
                    method="post"
                    enctype="multipart/form-data"
                    class="flex flex-col gap-4 justify-center items-center" {
                    div class="overflow-x-auto relative shadow-md rounded" {
                        table class="w-full" {
                            caption class="p-3 space-x-2 bg-white dark:bg-slate-800" {
                                a href="#add" class=(BUTTON_PRIMARY) { span class="w-4 h-4 i-tabler-plus"; }
                                button type="submit" name="submit" value="remove" class=(BUTTON_ERROR) {
                                    span class="w-4 h-4 i-tabler-trash";
                                }
                            }
                            thead class="
                                    text-xs
                                    text-gray-700
                                    uppercase
                                    dark:text-gray-400
                                    bg-slate-50
                                    dark:bg-slate-700
                                " {
                                tr {
                                    th class="p-3" { input type="checkbox" name="slugs_all" class="bg-transparent"; }
                                    th class="py-3 px-6" { "Name" }
                                }
                            }
                            tbody {
                                @for kind in actor_kinds {
                                    tr class="bg-white border-b last:border-0 dark:bg-slate-800 dark:border-slate-700" {
                                        td class="p-3" {
                                            input
                                                type="checkbox"
                                                name="slugs"
                                                value=(kind.slug)
                                                class="bg-transparent";
                                        }
                                        td class="py-3 px-6" {
                                            a
                                                href={"/games/" (game.slug) "/actors/" (kind.slug)}
                                                class="hover:text-violet-500" {
                                                (kind.name)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            li class="flex flex-col gap-2" {
                a href={"/games/" (game.slug) "/skills"} class="text-center hover:text-violet" { "Skills" }
            }
            li class="flex flex-col gap-2" {
                a href={"/games/" (game.slug) "/traits"} class="text-center hover:text-violet" { "Traits" }
            }
        }
    })
    .pre(html! {
        dialog id="edit" class=(DIALOG) {
            div class="flex z-10 flex-col gap-4 p-4 max-w-sm rounded border dark:text-white dark:bg-slate-900" {
                h2 class="text-xl" { "Edit Game" }
                form method="post" enctype="multipart/form-data" class="flex flex-col gap-4 justify-center" {
                    input
                        type="text"
                        name="name"
                        value=(game.name)
                        placeholder="Name"
                        required
                        autofocus
                        class="bg-transparent rounded invalid:border-red";
                    textarea
                        name="description"
                        value=(game.name)
                        placeholder="Description"
                        class="rounded invalid:border-red dark:bg-slate-900" {}
                    div class="flex justify-between" {
                        button type="submit" name="submit" value="edit" class=(BUTTON_SUCCESS) {
                            span class="w-4 h-4 i-tabler-check";
                        }
                        a href="#!" class=(BUTTON_PRIMARY) { span class="w-4 h-4 i-tabler-x"; }
                    }
                }
            }
            a href="#!" class="fixed inset-0" {}
        }
        dialog id="add" class=(DIALOG) {
            div class="flex z-10 flex-col gap-4 p-4 max-w-sm rounded border dark:text-white dark:bg-slate-900" {
                h2 class="text-xl" { "Add Actor Kind" }
                form method="post" enctype="multipart/form-data" class="flex flex-col gap-4 justify-center" {
                    input type="hidden" value=(game.id);
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
