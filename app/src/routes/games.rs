use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};
use axum_typed_multipart::{TryFromMultipart, TypedMultipart};
use maud::{html, Markup};
use sqlx::{Pool, Postgres};

use crate::{components::Page, AppState, BUTTON_ERROR, BUTTON_PRIMARY, BUTTON_SUCCESS, DIALOG};

pub mod game;

pub async fn games(pool: &Pool<Postgres>) -> Markup {
    let games = sqlx::query!("SELECT slug, name FROM game;").fetch_all(pool).await;

    Page::new(html! {
        h1 class="text-xl font-bold" { "Games" }
        form method="post" enctype="multipart/form-data" class="flex flex-col gap-4 justify-center items-center" {
            div class="overflow-x-auto relative shadow-md rounded" {
                table class="w-full" {
                    caption class="p-3 space-x-2 bg-white dark:bg-slate-800" {
                        a href="#add" class=(BUTTON_PRIMARY) { span class="w-4 h-4 i-tabler-plus"; }
                        button type="submit" name="submit" value="remove" class=(BUTTON_ERROR) {
                            span class="w-4 h-4 i-tabler-trash";
                        }
                    }
                    thead class="text-xs text-gray-700 uppercase dark:text-gray-400 bg-slate-50 dark:bg-slate-700" {
                        tr {
                            th class="p-3" { input type="checkbox" name="slugs_all" class="bg-transparent"; }
                            th class="py-3 px-6" { "Name" }
                        }
                    }
                    tbody {
                        @match games {
                            Ok(games) => {
                                @for game in games {
                                    tr class="bg-white border-b last:border-0 dark:bg-slate-800 dark:border-slate-700" {
                                        td class="p-3" {
                                            input
                                                type="checkbox"
                                                name="slugs"
                                                value=(game.slug)
                                                class="bg-transparent";
                                        }
                                        td class="py-3 px-6" {
                                            a href={"/games/" (game.slug)} class="hover:text-violet-500" {
                                                (game.name)
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
                h2 class="text-xl" { "Add Game" }
                form method="post" enctype="multipart/form-data" class="flex flex-col gap-4 justify-center" {
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

#[derive(TryFromMultipart)]
pub struct GamesPayload {
    pub submit: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub slugs_all: Option<bool>,
    pub slugs: Vec<String>,
}

pub async fn games_post(
    State(state): State<Arc<AppState>>,
    TypedMultipart(form): TypedMultipart<GamesPayload>,
) -> impl IntoResponse {
    match form.submit.as_str() {
        "add" => {
            sqlx::query!(
                "INSERT INTO game (name, description) VALUES ($1, $2);",
                form.name,
                form.description
            )
            .execute(&state.pool)
            .await
            .unwrap();
        }
        "remove" => {
            if form.slugs_all.is_some_and(|a| a) {
                sqlx::query!("DELETE FROM game;").execute(&state.pool).await.unwrap();
            } else {
                sqlx::query!("DELETE FROM game WHERE slug = ANY($1);", &form.slugs)
                    .execute(&state.pool)
                    .await
                    .unwrap();
            }
        }
        _ => {}
    };

    games(&state.pool).await
}

pub async fn games_get(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    games(&state.pool).await
}
