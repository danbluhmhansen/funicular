use std::sync::Arc;

use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use axum_extra::{extract::Form, routing::TypedPath};
use serde::Deserialize;
use strum::Display;

use crate::{
    components::{Dialog, Layout},
    routes, AppState,
};

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
            content: markup::new! {
                h1[class="text-xl font-bold"] { "Games" }
                div[class="overflow-x-auto relative rounded shadow-md"] {
                    form[method="post"] {
                        div[class="flex flex-row gap-2 justify-center p-3 bg-white dark:bg-slate-800"] {
                            a[href={format!("#{}", Submit::Add)},class="btn-primary","hx-boost"="false"] {
                                span[class="w-4 h-4 i-tabler-plus"];
                            }
                            button[type="submit",name="submit",value=Submit::Remove.to_string(),class="btn-error"] {
                                span[class="w-4 h-4 i-tabler-trash"];
                            }
                        }
                        div[
                            "hx-get"=routes::partials::games_table::Path.to_string(),
                            "hx-trigger"="revealed",
                            "hx-select"="#games-table",
                            "hx-target"="this",
                        ] {
                            span[class="w-6 h-6 i-svg-spinners-gooey-balls-2"];
                        }
                    }
                }
                @Dialog {
                    id: Submit::Add,
                    title: "Add Game",
                    content: markup::new! {
                        form[method="post",class="flex flex-col gap-4 justify-center"] {
                            input[
                                type="text",
                                name="name",
                                placeholder="Name",
                                required,
                                autofocus,
                                class="bg-transparent rounded invalid:border-red"
                            ];
                            textarea[
                                name="description",
                                placeholder="Description",
                                class="bg-transparent rounded invalid:border-red"
                            ] {}
                            div[class="flex justify-between"] {
                                button[type="submit",name="submit",value=Submit::Add.to_string(),class="btn-primary"] {
                                    span[class="w-4 h-4 i-tabler-check"];
                                }
                            }
                        }
                    }
                }
            },
        }
        .to_string(),
    )
}

#[derive(Deserialize)]
pub(crate) struct Payload {
    pub(crate) submit: Submit,
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    #[serde(default)]
    pub(crate) slugs: Vec<String>,
}

pub(crate) async fn post(
    path: Path,
    State(state): State<Arc<AppState>>,
    Form(form): Form<Payload>,
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
