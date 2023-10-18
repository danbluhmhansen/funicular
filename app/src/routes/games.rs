use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};
use axum_extra::{extract::Form, routing::TypedPath};
use maud::html;
use serde::Deserialize;
use strum::Display;

use crate::{components::layout, AppState};

pub(crate) mod game;

#[derive(Deserialize, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub(crate) enum Submit {
    Add,
    Remove,
}

#[derive(TypedPath)]
#[typed_path("/games")]
pub(crate) struct Path;

pub(crate) async fn get(_: Path) -> impl IntoResponse {
    layout(html! {
        h1 class="text-xl font-bold" { "Games" }
        div class="overflow-x-auto relative rounded shadow-md" {
            form method="post" {
                div class="flex flex-row gap-2 justify-center p-3 bg-white dark:bg-slate-800" {
                    a href={"#" (Submit::Add)} class="btn-primary" hx-boost="false" {
                        span class="w-4 h-4 i-tabler-plus";
                    }
                    button type="submit" name="submit" value=(Submit::Remove) class="btn-error" {
                        span class="w-4 h-4 i-tabler-trash";
                    }
                }
                div hx-get=(crate::routes::partials::games_table::Path) hx-trigger="revealed" hx-swap="outerHTML" {
                    span class="w-6 h-6 i-svg-spinners-gooey-balls-2";
                }
            }
        }
        dialog
            id=(Submit::Add)
            class="hidden inset-0 z-10 justify-center items-center w-full h-full target:flex bg-black/50 backdrop-blur-sm" {
            div class="flex z-10 flex-col gap-4 p-4 max-w-sm bg-white rounded border dark:text-white dark:bg-slate-900" {
                div {
                    a href="#!" hx-boost="false" class="float-right w-4 h-4 i-tabler-x" {}
                    h2 class="text-xl" { "Add Game" }
                }
                form method="post" class="flex flex-col gap-4 justify-center" {
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
                        class="bg-transparent rounded invalid:border-red" {}
                    div class="flex justify-between" {
                        button type="submit" name="submit" value=(Submit::Add) class="btn-primary" {
                            span class="w-4 h-4 i-tabler-check";
                        }
                    }
                }
            }
            a href="#!" hx-boost="false" class="fixed inset-0" {}
        }
    })
}

#[derive(Deserialize)]
pub(crate) struct GamesForm {
    pub(crate) submit: Submit,
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    #[serde(default)]
    pub(crate) slugs: Vec<String>,
}

pub(crate) async fn post(
    path: Path,
    State(state): State<Arc<AppState>>,
    Form(form): Form<GamesForm>,
) -> impl IntoResponse {
    match form.submit {
        Submit::Add => {
            let res = sqlx::query!(
                "INSERT INTO game (name, description) VALUES ($1, $2);",
                form.name,
                form.description
            )
            .execute(&state.pool)
            .await;
        }
        Submit::Remove => {
            let res = sqlx::query!("DELETE FROM game WHERE slug = ANY($1);", &form.slugs)
                .execute(&state.pool)
                .await;
        }
    }

    get(path).await
}
