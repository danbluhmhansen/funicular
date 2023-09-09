use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect, Response},
};
use axum_typed_multipart::{TryFromField, TryFromMultipart, TypedMultipart};
use maud::html;
use sqlx::{Pool, Postgres};
use strum::Display;

use crate::{
    components::Page, routes::not_found, AppState, BUTTON_ERROR, BUTTON_PRIMARY, BUTTON_SUCCESS, BUTTON_WARNING,
    CAPTION, DIALOG,
};

pub mod actors;
pub mod skills;
pub mod traits;

#[derive(Display, TryFromField)]
#[strum(serialize_all = "snake_case")]
#[try_from_field(rename_all = "snake_case")]
pub enum Submit {
    Add,
    Edit,
    Remove,
}

async fn game(game_slug: String, pool: &Pool<Postgres>) -> Response {
    let game = sqlx::query!(
        "SELECT id, name, slug, description FROM game WHERE slug = $1;",
        game_slug
    )
    .fetch_one(pool)
    .await;

    if game.is_err() {
        return not_found().await.into_response();
    }

    let game = game.unwrap();

    let actor_kinds = sqlx::query!(
        r#"
            SELECT actor_kind.name, actor_kind.slug
            FROM actor_kind
            JOIN game ON game.id = actor_kind.game_id
            WHERE game.slug = $1
        "#,
        game_slug
    )
    .fetch_all(pool)
    .await;

    Page::new(html! {
        div class="flex flex-row gap-2 justify-center items-center" {
            h1 class="text-xl font-bold" { (game.name) }
            a href={"#" (Submit::Edit)} class=(BUTTON_WARNING) { span class="w-4 h-4 i-tabler-pencil"; }
        }
        ul class="flex flex-col gap-4" {
            li class="flex flex-col gap-2" {
                h2 class="text-center" { "Actors" }
                form
                    method="post"
                    enctype="multipart/form-data"
                    class="flex flex-col gap-4 justify-center items-center" {
                    input type="hidden" name="game_id" value=(game.id);
                    div class="overflow-x-auto relative shadow-md rounded w-96" {
                        table class="w-full" {
                            caption class=(CAPTION) {
                                a href={"#" (Submit::Add)} class=(BUTTON_PRIMARY) { span class="w-4 h-4 i-tabler-plus"; }
                                button type="submit" name="submit" value=(Submit::Remove) class=(BUTTON_ERROR) {
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
                                    th class="p-3 text-center" {
                                        input type="checkbox" name="slugs_all" value="true" class="bg-transparent";
                                    }
                                    th class="py-3 px-6 text-left" { "Name" }
                                }
                            }
                            tbody {
                                @match actor_kinds {
                                    Ok(actor_kinds) => {
                                        @for kind in actor_kinds {
                                            tr class="
                                                bg-white
                                                border-b
                                                last:border-0
                                                dark:bg-slate-800
                                                dark:border-slate-700
                                            " {
                                                td class="p-3 text-center" {
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
                                    Err(_) => {
                                        p { "No actor kinds..." }
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
        dialog id=(Submit::Edit) class=(DIALOG) {
            div class="flex z-10 flex-col gap-4 p-4 max-w-sm rounded border dark:text-white dark:bg-slate-900" {
                h2 class="text-xl" { "Edit Game" }
                form method="post" enctype="multipart/form-data" class="flex flex-col gap-4 justify-center" {
                    input type="hidden" name="game_id" value=(game.id);
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
                        button type="submit" name="submit" value=(Submit::Edit) class=(BUTTON_SUCCESS) {
                            span class="w-4 h-4 i-tabler-check";
                        }
                        a href="#!" class=(BUTTON_PRIMARY) { span class="w-4 h-4 i-tabler-x"; }
                    }
                }
            }
            a href="#!" class="fixed inset-0" {}
        }
        dialog id=(Submit::Add) class=(DIALOG) {
            div class="flex z-10 flex-col gap-4 p-4 max-w-sm rounded border dark:text-white dark:bg-slate-900" {
                h2 class="text-xl" { "Add Actor Kind" }
                form method="post" enctype="multipart/form-data" class="flex flex-col gap-4 justify-center" {
                    input type="hidden" name="game_id" value=(game.id);
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
                        button type="submit" name="submit" value=(Submit::Add) class=(BUTTON_SUCCESS) {
                            span class="w-4 h-4 i-tabler-check";
                        }
                        a href="#!" class=(BUTTON_PRIMARY) { span class="w-4 h-4 i-tabler-x"; }
                    }
                }
            }
            a href="#!" class="fixed inset-0" {}
        }
    })
    .build().into_response()
}

pub async fn game_get(Path(game_slug): Path<String>, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    game(game_slug, &state.pool).await
}

#[derive(TryFromMultipart)]
pub struct Payload {
    pub submit: Submit,
    pub name: Option<String>,
    pub description: Option<String>,
    pub game_id: Option<uuid::Uuid>,
    pub slugs_all: Option<bool>,
    pub slugs: Vec<String>,
}

pub async fn game_post(
    Path(game_slug): Path<String>,
    State(state): State<Arc<AppState>>,
    TypedMultipart(form): TypedMultipart<Payload>,
) -> Response {
    match form.submit {
        Submit::Edit => {
            let game_res = sqlx::query!(
                "UPDATE game SET name = $1, description = $2 WHERE id = $3 RETURNING slug;",
                form.name,
                form.description,
                form.game_id
            )
            .fetch_one(&state.pool)
            .await;

            if game_res.is_err() {
                todo!()
            }

            let new_slug = game_res.unwrap().slug;

            if game_slug != new_slug {
                Redirect::to(&format!("/games/{}", new_slug)).into_response()
            } else {
                game(game_slug, &state.pool).await.into_response()
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

            game(game_slug, &state.pool).await.into_response()
        }
        Submit::Remove => {
            if form.slugs_all.is_some_and(|a| a) {
                _ = sqlx::query!("DELETE FROM actor_kind WHERE game_id = $1;", form.game_id)
                    .execute(&state.pool)
                    .await;
            } else {
                let res = sqlx::query!(
                    "DELETE FROM actor_kind WHERE game_id = $1 AND slug = ANY($2);",
                    form.game_id,
                    &form.slugs
                )
                .execute(&state.pool)
                .await;
            }

            game(game_slug, &state.pool).await.into_response()
        }
    }
}
