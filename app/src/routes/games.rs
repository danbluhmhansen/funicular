use std::sync::Arc;

use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use axum_extra::{extract::Form, routing::TypedPath};
use serde::Deserialize;
use strum::Display;

use crate::{components::Layout, AppState, BUTTON_ERROR, BUTTON_PRIMARY};

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
    Html(
        Layout {
            main: markup::new! {
                h1."text-xl"."font-bold" { "Games" }
                ."overflow-x-auto".relative.rounded."shadow-md" {
                    form[method="post"] {
                        .flex."flex-row"."gap-2"."justify-center"."p-3"."bg-white"."dark:bg-slate-800" {
                            a[
                                href={format!("#{}", Submit::Add)},
                                class={BUTTON_PRIMARY},
                                "hx-boost"="false"
                            ] { ."w-4"."h-4"."i-tabler-plus"{} }
                            button[type="submit",name="submit",value={Submit::Remove.to_string()},class={BUTTON_ERROR}] {
                                ."w-4"."h-4"."i-tabler-trash"{}
                            }
                        }
                        div[
                            "hx-get"={crate::routes::partials::games_table::Path.to_string()},
                            "hx-trigger"="revealed","hx-swap"="outerHTML"
                        ] {
                            ."w-6"."h-6"."i-svg-spinners-gooey-balls-2"{}
                        }
                    }
                }
                dialog
                    .hidden
                    ."inset-0"
                    ."z-10"
                    ."justify-center"
                    ."items-center"
                    ."w-full"
                    ."h-full"
                    ."target:flex"
                    ."bg-black/50"
                    ."backdrop-blur-sm"
                    [id={Submit::Add.to_string()}] {
                    .flex
                    ."z-10"
                    ."flex-col"
                    ."gap-4"
                    ."p-4"
                    ."max-w-sm"
                    ."bg-white"
                    .rounded
                    .border
                    ."dark:text-white"
                    ."dark:bg-slate-900" {
                        div {
                            a."float-right"."w-4"."h-4"."i-tabler-x"[href="#!","hx-boost"="false"]{}
                            h2."text-xl" { "Add Game" }
                        }
                        form.flex."flex-col"."gap-4"."justify-center"[method="post"] {
                            input."bg-transparent".rounded."invalid:border-red"[
                                type="text",
                                name="name",
                                placeholder="Name",
                                required,
                                autofocus
                            ];
                            textarea."bg-transparent".rounded."invalid:border-red"[
                                name="description",
                                placeholder="Description"
                            ]{}
                            .flex."justify-between" {
                                button[type="submit",name="submit",value={Submit::Add.to_string()},class={BUTTON_PRIMARY}] {
                                    ."w-4"."h-4"."i-tabler-check"{}
                                }
                            }
                        }
                    }
                    a.fixed."inset-0"[href="#!","hx-boost"="false"]{}
                }
            },
        }
        .to_string(),
    )
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
