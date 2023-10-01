use std::sync::Arc;

use axum::{
    extract::State,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::routing::TypedPath;
use axum_typed_multipart::{TryFromField, TryFromMultipart, TypedMultipart};
use maud::html;
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use strum::Display;

use crate::{
    components::{
        dialog::Dialog,
        table::{Table, TableData, TableHead},
        Page,
    },
    routes::{self, not_found},
    AppState, BUTTON_ERROR, BUTTON_PRIMARY, BUTTON_SUCCESS, BUTTON_WARNING,
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
        "
        SELECT game.id, game.name, game.slug, game.description,
            JSONB_AGG(JSON_BUILD_OBJECT('name', actor_kind.name, 'slug', actor_kind.slug)) AS actor_kinds
        FROM game
        JOIN actor_kind ON actor_kind.game_id = game.id
        WHERE game.slug = $1
        GROUP BY game.id;
        ",
        game_slug
    )
    .fetch_one(pool)
    .await;

    if game.is_err() {
        return not_found().await.into_response();
    }

    let game = game.unwrap();

    let actor_kinds = game.actor_kinds.map_or(vec![], |actor_kinds| {
        actor_kinds.as_array().map_or(vec![], |actor_kinds| {
            actor_kinds
                .into_iter()
                .map(|actor_kind| {
                    vec![
                        TableData::Checkbox("slugs", actor_kind["slug"].as_str().map(|s| s.to_string())),
                        TableData::Data(html! {
                            a
                                href=(routes::games::game::actors::Path::new(
                                    // TODO: avoid clone
                                    game.slug.clone(),
                                    actor_kind["slug"].as_str().unwrap_or("").to_string()
                                ))
                                class="hover:text-violet-500" {
                                (actor_kind["name"].as_str().unwrap_or(""))
                            }
                        }),
                    ]
                })
                .collect::<Vec<Vec<TableData>>>()
        })
    });

    Page::new(html! {
        div class="flex flex-row gap-2 justify-center items-center" {
            h1 class="text-xl font-bold" { (game.name) }
            a href={"#" (Submit::Edit)} class=(BUTTON_WARNING) { span class="w-4 h-4 i-tabler-pencil" {} }
        }
        @if let Some(description) = &game.description { p { (description) } }
        ul class="flex flex-col gap-4" {
            li class="flex flex-col gap-2" {
                h2 class="text-center" { "Actors" }
                form
                    method="post"
                    enctype="multipart/form-data"
                    class="flex flex-col gap-4 justify-center items-center" {
                    input type="hidden" name="game_id" value=(game.id);
                    (Table::new()
                        .caption(html! {
                            a href={"#" (Submit::Add)} class=(BUTTON_PRIMARY) { span class="w-4 h-4 i-tabler-plus" {} }
                            button type="submit" name="submit" value=(Submit::Remove) class=(BUTTON_ERROR) {
                                span class="w-4 h-4 i-tabler-trash" {}
                            }
                        })
                        .head(TableHead::Checkbox("slugs_all"))
                        .head(TableHead::Header(html! { "Name" }))
                        .body_or(actor_kinds, html! { p { "No actor kinds..." } })
                    )
                }
            }
            li class="flex flex-col gap-2" {
                a
                    // TODO: avoid clone
                    href=(routes::games::game::skills::Path::new(game.slug.clone()))
                    class="text-center hover:text-violet"
                { "Skills" }
            }
            li class="flex flex-col gap-2" {
                a
                    href=(routes::games::game::traits::Path::new(game.slug))
                    class="text-center hover:text-violet"
                { "Traits" }
            }
        }
    })
    .dialog(
        Dialog::new(html! {
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
                    placeholder="Description"
                    class="bg-transparent rounded invalid:border-red" {
                        @if let Some(description) = &game.description { (description) }
                    }
                div class="flex justify-between" {
                    button type="submit" name="submit" value=(Submit::Edit) class=(BUTTON_SUCCESS) {
                        span class="w-4 h-4 i-tabler-check" {}
                    }
                }
            }
        })
        .id(Submit::Edit)
        .title("Edit Game"),
    )
    .dialog(
        Dialog::new(html! {
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
                    class="bg-transparent rounded invalid:border-red" {}
                div class="flex justify-between" {
                    button type="submit" name="submit" value=(Submit::Add) class=(BUTTON_SUCCESS) {
                        span class="w-4 h-4 i-tabler-check" {}
                    }
                }
            }
        })
        .id(Submit::Add)
        .title("Add Actor Kind"),
    )
    .into_response()
}

#[derive(Deserialize, TypedPath)]
#[typed_path("/games/:game_slug")]
pub struct Path {
    pub game_slug: String,
}

impl Path {
    pub fn new(game_slug: String) -> Self {
        Self { game_slug }
    }
}

pub async fn get(Path { game_slug }: Path, State(state): State<Arc<AppState>>) -> impl IntoResponse {
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

pub async fn post(
    Path { game_slug }: Path,
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
                Redirect::to(&format!("/games/{new_slug}")).into_response()
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
