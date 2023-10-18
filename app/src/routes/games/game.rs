use std::sync::Arc;

use axum::{
    extract::State,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::{extract::Form, routing::TypedPath};
use maud::html;
use serde::Deserialize;
use strum::Display;
use uuid::Uuid;

use crate::{
    components::{layout, not_found},
    AppState,
};

#[derive(Deserialize, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub(crate) enum Submit {
    Edit,
    Add,
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

pub(crate) async fn get(Path { game_slug }: Path, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    if let Ok(game) = sqlx::query!("SELECT id, name, description FROM game WHERE slug = $1;", *game_slug)
        .fetch_one(&state.pool)
        .await
    {
        layout(html! {
            div class="flex flex-row gap-2 justify-center items-center" {
                h1 class="text-xl font-bold" { (&game.name) }
                a href={"#" (Submit::Edit)} class="btn-warning" {
                    span class="w-4 h-4 i-tabler-pencil";
                }
            }
            div class="overflow-x-auto relative rounded shadow-md" {
                form method="post" {
                    input type="hidden" name="game_id" value=(&game.id);
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
                    form method="post" class="flex flex-col gap-4 justify-center" {
                        input
                            type="text"
                            name="name"
                            placeholder="Name"
                            required
                            autofocus
                            value=(&game.name)
                            class="bg-transparent rounded invalid:border-red";
                        textarea
                            name="description"
                            placeholder="Description"
                            value=[&game.description]
                            class="bg-transparent rounded invalid:border-red" {}
                        div class="flex justify-between" {
                            button type="submit" name="submit" value=(Submit::Edit) class="btn-primary" {
                                span class="w-4 h-4 i-tabler-check";
                            }
                        }
                    }
                }
                a href="#!" hx-boost="false" class="fixed inset-0" {}
            }
            dialog
                id=(Submit::Add)
                class="hidden inset-0 z-10 justify-center items-center w-full h-full target:flex bg-black/50 backdrop-blur-sm" {
                div class="flex z-10 flex-col gap-4 p-4 max-w-sm bg-white rounded border dark:text-white dark:bg-slate-900" {
                    div {
                        a href="#!" hx-boost="false" class="float-right w-4 h-4 i-tabler-x" {}
                        h2 class="text-xl" { "Add Actor Kind" }
                    }
                    form method="post" class="flex flex-col gap-4 justify-center" {
                        input type="hidden" name="game_id" value=(&game.id);
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
    } else {
        layout(not_found())
    }
}

#[derive(Deserialize)]
pub(crate) struct Payload {
    pub(crate) submit: Submit,
    pub(crate) game_id: Option<Uuid>,
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    #[serde(default)]
    pub(crate) slugs: Vec<String>,
}

pub(crate) async fn post(path: Path, State(state): State<Arc<AppState>>, Form(form): Form<Payload>) -> Response {
    match form.submit {
        Submit::Edit => {
            match sqlx::query!(
                "UPDATE game SET name = $1, description = $2 WHERE slug = $3 RETURNING slug;",
                form.name,
                form.description,
                *path.game_slug
            )
            .fetch_one(&state.pool)
            .await
            {
                Ok(game) if *path.game_slug != game.slug => {
                    Redirect::to(&format!("/games/{}", game.slug)).into_response()
                }
                _ => get(path, State(state)).await.into_response(),
            }
        }
        Submit::Add => {
            let res = sqlx::query!(
                "INSERT INTO actor_kind (game_id, name, description) VALUES ($1, $2, $3);",
                form.game_id,
                form.name,
                form.description
            )
            .execute(&state.pool)
            .await;
            get(path, State(state)).await.into_response()
        }
        Submit::Remove => {
            let res = sqlx::query!(
                "DELETE FROM actor_kind WHERE game_id = $1 AND slug = ANY($2);",
                form.game_id,
                &form.slugs
            )
            .execute(&state.pool)
            .await;
            get(path, State(state)).await.into_response()
        }
    }
}
