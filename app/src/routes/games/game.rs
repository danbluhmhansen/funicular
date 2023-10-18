use std::sync::Arc;

use axum::response::IntoResponse;
use axum_extra::routing::TypedPath;
use maud::html;
use serde::Deserialize;
use strum::Display;

use crate::components::layout;

#[derive(Deserialize, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub(crate) enum Submit {
    Add,
    Edit,
    Remove,
}

#[derive(Deserialize, TypedPath)]
#[typed_path("/games/:game_slug")]
pub(crate) struct Path {
    game_slug: Arc<String>,
}

impl Path {
    pub(crate) fn new(game_slug: Arc<String>) -> Self {
        Self { game_slug }
    }
}

pub(crate) async fn get(Path { game_slug }: Path) -> impl IntoResponse {
    layout(html! {
        div
            hx-get=(crate::routes::partials::game_name::Path::new(game_slug.clone()))
            hx-trigger="revealed"
            hx-swap="outerHTML" {
            span class="w-6 h-6 i-svg-spinners-gooey-balls-2";
        }
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
                div
                    hx-get=(crate::routes::partials::actor_kinds_table::Path::new(game_slug.clone()))
                    hx-trigger="revealed"
                    hx-swap="outerHTML" {
                    span class="w-6 h-6 i-svg-spinners-gooey-balls-2";
                }
            }
        }
        dialog
            id=(Submit::Edit)
            class="hidden inset-0 z-10 justify-center items-center w-full h-full target:flex bg-black/50 backdrop-blur-sm" {
            div class="flex z-10 flex-col gap-4 p-4 max-w-sm bg-white rounded border dark:text-white dark:bg-slate-900" {
                div {
                    a href="#!" hx-boost="false" class="float-right w-4 h-4 i-tabler-x" {}
                    h2 class="text-xl" { "Edit Game" }
                }
                // TODO: trigger load on dialog show instead of revealed
                div
                    hx-get=(crate::routes::partials::game_form::Path::new(game_slug.clone()))
                    hx-trigger="revealed"
                    hx-swap="outerHTML" {
                    span class="w-6 h-6 i-svg-spinners-gooey-balls-2";
                }
            }
            a href="#!" hx-boost="false" class="fixed inset-0" {}
        }
    })
}
