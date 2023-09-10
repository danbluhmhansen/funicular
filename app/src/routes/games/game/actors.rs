use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use axum_typed_multipart::{TryFromField, TryFromMultipart, TypedMultipart};
use maud::html;
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use strum::Display;

use crate::{
    components::{Dialog, Page},
    routes::not_found,
    AppState, BUTTON_ERROR, BUTTON_PRIMARY, BUTTON_SUCCESS, CAPTION, DIALOG, THEAD, TR,
};

pub mod actor;

#[derive(Deserialize)]
pub struct ActorsPath {
    pub game_slug: String,
    pub actor_kind_slug: String,
}

#[derive(Display, TryFromField)]
#[strum(serialize_all = "snake_case")]
#[try_from_field(rename_all = "snake_case")]
pub enum Submit {
    Add,
    Remove,
}

async fn actors(game_slug: String, actor_kind_slug: String, pool: &Pool<Postgres>) -> Response {
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

    let actor_kind = sqlx::query!("SELECT id, name FROM actor_kind WHERE slug = $1", actor_kind_slug)
        .fetch_one(pool)
        .await;

    if actor_kind.is_err() {
        return not_found().await.into_response();
    }

    let actor_kind = actor_kind.unwrap();

    let actors = sqlx::query!(
        r#"
            SELECT actor.name, actor.slug
            FROM actor
            JOIN actor_kind ON actor_kind.id = actor.kind_id
            WHERE actor_kind.slug = $1;
        "#,
        actor_kind_slug
    )
    .fetch_all(pool)
    .await;

    Page::new(html! {
        a href={"/games/" (game_slug)} class="text-xl font-bold hover:text-violet-500" { (game.name) }
        form method="post" enctype="multipart/form-data" class="flex flex-col gap-4 justify-center items-center" {
            input type="hidden" name="kind_id" value=(actor_kind.id);
            div class="overflow-x-auto relative rounded shadow-md" {
                table class="w-full" {
                    caption class=(CAPTION) {
                        a href={"#" (Submit::Add)} class=(BUTTON_PRIMARY) { span class="w-4 h-4 i-tabler-plus" {} }
                        button type="submit" name="submit" value=(Submit::Remove) class=(BUTTON_ERROR) {
                            span class="w-4 h-4 i-tabler-trash" {}
                        }
                    }
                    thead class=(THEAD) {
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
                                    tr class=(TR) {
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
    .dialog(
        Dialog::new(html! {
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
                    class="bg-transparent rounded invalid:border-red" {}
                div class="flex justify-between" {
                    button type="submit" name="submit" value=(Submit::Add) class=(BUTTON_SUCCESS) {
                        span class="w-4 h-4 i-tabler-check" {}
                    }
                }
            }
        })
        .id(Submit::Add)
        .title(&format!("Add {}", actor_kind.name))
    )
    .render().into_response()
}

pub async fn actors_get(
    Path(ActorsPath {
        game_slug,
        actor_kind_slug,
    }): Path<ActorsPath>,
    State(state): State<Arc<AppState>>,
) -> Response {
    actors(game_slug, actor_kind_slug, &state.pool).await
}

#[derive(TryFromMultipart)]
pub struct Payload {
    pub submit: Submit,
    pub kind_id: Option<uuid::Uuid>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub slugs_all: Option<bool>,
    pub slugs: Vec<String>,
}

pub async fn actors_post(
    Path(ActorsPath {
        game_slug,
        actor_kind_slug,
    }): Path<ActorsPath>,
    State(state): State<Arc<AppState>>,
    TypedMultipart(form): TypedMultipart<Payload>,
) -> Response {
    match form.submit {
        Submit::Add => {
            let res = sqlx::query!(
                "INSERT INTO actor (kind_id, name, description) VALUES ($1, $2, $3);",
                form.kind_id,
                form.name,
                form.description
            )
            .execute(&state.pool)
            .await;

            actors(game_slug, actor_kind_slug, &state.pool).await
        }
        Submit::Remove => {
            if form.slugs_all.is_some_and(|a| a) {
                _ = sqlx::query!("DELETE FROM actor WHERE kind_id = $1;", form.kind_id)
                    .execute(&state.pool)
                    .await;
            } else {
                let res = sqlx::query!(
                    "DELETE FROM actor WHERE kind_id = $1 AND slug = ANY($2);",
                    form.kind_id,
                    &form.slugs
                )
                .execute(&state.pool)
                .await;
            }

            actors(game_slug, actor_kind_slug, &state.pool).await
        }
    }
}
