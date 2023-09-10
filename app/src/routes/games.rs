use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};
use axum_typed_multipart::{TryFromField, TryFromMultipart, TypedMultipart};
use maud::{html, Markup};
use sqlx::{Pool, Postgres};
use strum::Display;

use crate::{
    components::{Dialog, Page},
    AppState, BUTTON_ERROR, BUTTON_PRIMARY, BUTTON_SUCCESS, CAPTION, DIALOG, THEAD, TR,
};

pub mod game;

#[derive(Display, TryFromField)]
#[strum(serialize_all = "snake_case")]
#[try_from_field(rename_all = "snake_case")]
pub enum Submit {
    Add,
    Remove,
}

async fn games(pool: &Pool<Postgres>) -> Markup {
    let games = sqlx::query!("SELECT slug, name FROM game;").fetch_all(pool).await;

    Page::new(html! {
        h1 class="text-xl font-bold" { "Games" }
        form method="post" enctype="multipart/form-data" class="flex flex-col gap-4 justify-center items-center" {
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
                        @match games {
                            Ok(games) => {
                                @for game in games {
                                    tr class=(TR) {
                                        td class="p-3 text-center" {
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
    .dialog(
        Dialog::new(html! {
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
                    class="bg-transparent rounded invalid:border-red" {}
                div class="flex justify-between" {
                    button type="submit" name="submit" value=(Submit::Add) class=(BUTTON_SUCCESS) {
                        span class="w-4 h-4 i-tabler-check" {}
                    }
                }
            }
        })
        .id(Submit::Add)
        .title("Add Game"),
    )
    .render()
}

#[derive(TryFromMultipart)]
pub struct Payload {
    pub submit: Submit,
    pub name: Option<String>,
    pub description: Option<String>,
    pub slugs_all: Option<bool>,
    pub slugs: Vec<String>,
}

pub async fn games_post(
    State(state): State<Arc<AppState>>,
    TypedMultipart(form): TypedMultipart<Payload>,
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
            if form.slugs_all.is_some_and(|a| a) {
                _ = sqlx::query!("DELETE FROM game;").execute(&state.pool).await;
            } else {
                let res = sqlx::query!("DELETE FROM game WHERE slug = ANY($1);", &form.slugs)
                    .execute(&state.pool)
                    .await;
            }
        }
    };

    games(&state.pool).await
}

pub async fn games_get(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    games(&state.pool).await
}
