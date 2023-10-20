use std::sync::Arc;

use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect, Response},
};
use axum_extra::{extract::Form, routing::TypedPath};
use serde::Deserialize;
use strum::Display;
use uuid::Uuid;

use crate::{
    components::{Layout, NotFound},
    AppState,
};

pub(crate) mod actor;

#[derive(Deserialize, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub(crate) enum Submit {
    Edit,
    Add,
    Remove,
}

#[derive(Deserialize, TypedPath)]
#[typed_path("/games/:game_slug/actors/:actor_kind_slug")]
pub(crate) struct Path {
    game_slug: Arc<String>,
    actor_kind_slug: Arc<String>,
}

impl Path {
    pub(crate) fn new(game_slug: Arc<String>, actor_kind_slug: Arc<String>) -> Self {
        Self {
            game_slug,
            actor_kind_slug,
        }
    }
}

pub(crate) async fn get(
    Path {
        game_slug,
        actor_kind_slug,
    }: Path,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    if let Ok(actor_kind) = sqlx::query!(
        "
        SELECT actor_kind.id, actor_kind.name, actor_kind.description FROM actor_kind
        JOIN game ON game.id = actor_kind.game_id
        WHERE game.slug = $1 AND actor_kind.slug = $2;
        ",
        *game_slug,
        *actor_kind_slug
    )
    .fetch_one(&state.pool)
    .await
    {
        Html(Layout { content: markup::new! {
            div[class="flex flex-row gap-2 justify-center items-center"] {
                h1[class="text-xl font-bold"] { @actor_kind.name }
                a[href=format!("#{}", Submit::Edit),class="btn-warning"] {
                    span[class="w-4 h-4 i-tabler-pencil"];
                }
            }
            div[class="overflow-x-auto relative rounded shadow-md"] {
                form[method="post"] {
                    input[type="hidden",name="actor_kind_id",value=&actor_kind.id.to_string()];
                    div[class="flex flex-row gap-2 justify-center p-3 bg-white dark:bg-slate-800"] {
                        a[href=format!("#{}", Submit::Add),class="btn-primary","hx-boost"="false"] {
                            span[class="w-4 h-4 i-tabler-plus"];
                        }
                        button[type="submit",name="submit",value=Submit::Remove.to_string(),class="btn-error"] {
                            span[class="w-4 h-4 i-tabler-trash"];
                        }
                    }
                    div[
                        "hx-get"=crate::routes::partials::actors_table::Path::new(
                            game_slug.clone(),
                            actor_kind_slug.clone()
                        ).to_string(),
                        "hx-trigger"="revealed",
                        "hx-swap"="outerHTML"
                    ] {
                        span[class="w-6 h-6 i-svg-spinners-gooey-balls-2"];
                    }
                }
            }
            dialog[
                id=Submit::Edit.to_string(),
                class="hidden inset-0 z-10 justify-center items-center w-full h-full target:flex bg-black/50 backdrop-blur-sm"
            ] {
                div[class="flex z-10 flex-col gap-4 p-4 max-w-sm bg-white rounded border dark:text-white dark:bg-slate-900"] {
                    div {
                        a[href="#!","hx-boost"="false",class="float-right w-4 h-4 i-tabler-x"] {}
                        h2[class="text-xl"] { "Edit Actor Kind" }
                    }
                    form[method="post",class="flex flex-col gap-4 justify-center"] {
                        input[type="hidden",name="kind_id",value=actor_kind.id.to_string()];
                        input[
                            type="text",
                            name="name",
                            placeholder="Name",
                            required,
                            autofocus,
                            value=&actor_kind.name,
                            class="bg-transparent rounded invalid:border-red"
                        ];
                        textarea[
                            name="description",
                            placeholder="Description",
                            value=&actor_kind.description,
                            class="bg-transparent rounded invalid:border-red"
                        ] {}
                        div[class="flex justify-between"] {
                            button[type="submit",name="submit",value=Submit::Edit.to_string(),class="btn-primary"] {
                                span[class="w-4 h-4 i-tabler-check"];
                            }
                        }
                    }
                }
                a[href="#!","hx-boost"="false",class="fixed inset-0"] {}
            }
            dialog[
                id=Submit::Add.to_string(),
                class="hidden inset-0 z-10 justify-center items-center w-full h-full target:flex bg-black/50 backdrop-blur-sm"
            ] {
                div[class="flex z-10 flex-col gap-4 p-4 max-w-sm bg-white rounded border dark:text-white dark:bg-slate-900"] {
                    div {
                        a[href="#!","hx-boost"="false",class="float-right w-4 h-4 i-tabler-x"] {}
                        h2[class="text-xl"] { "Add Actor Kind" }
                    }
                    form[method="post",class="flex flex-col gap-4 justify-center"] {
                        input[type="hidden",name="kind_id",value=actor_kind.id.to_string()];
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
                a[href="#!","hx-boost"="false",class="fixed inset-0"] {}
            }
        }}.to_string())
    } else {
        Html(Layout { content: NotFound {} }.to_string())
    }
}

#[derive(Deserialize)]
pub(crate) struct Payload {
    pub(crate) submit: Submit,
    pub(crate) kind_id: Option<Uuid>,
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    #[serde(default)]
    pub(crate) slugs: Vec<String>,
}

pub(crate) async fn post(path: Path, State(state): State<Arc<AppState>>, Form(form): Form<Payload>) -> Response {
    match form.submit {
        Submit::Edit => {
            match sqlx::query!(
                "UPDATE actor_kind SET name = $1, description = $2 WHERE id = $3 RETURNING slug;",
                form.name,
                form.description,
                form.kind_id.unwrap_or_default()
            )
            .fetch_one(&state.pool)
            .await
            {
                Ok(actor_kind) if *path.actor_kind_slug != actor_kind.slug => {
                    Redirect::to(&Path::new(path.game_slug, Arc::new(actor_kind.slug)).to_string()).into_response()
                }
                _ => get(path, State(state)).await.into_response(),
            }
        }
        Submit::Add => {
            let res = sqlx::query!(
                "INSERT INTO actor (kind_id, name, description) VALUES ($1, $2, $3);",
                form.kind_id,
                form.name,
                form.description
            )
            .execute(&state.pool)
            .await;
            get(path, State(state)).await.into_response()
        }
        Submit::Remove => {
            let res = sqlx::query!(
                "DELETE FROM actor WHERE kind_id = $1 AND slug = ANY($2);",
                form.kind_id,
                &form.slugs
            )
            .execute(&state.pool)
            .await;
            get(path, State(state)).await.into_response()
        }
    }
}
