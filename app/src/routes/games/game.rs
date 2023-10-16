use std::sync::Arc;

use axum::response::{Html, IntoResponse};
use axum_extra::routing::TypedPath;
use serde::Deserialize;
use strum::Display;

use crate::{components::Layout, BUTTON_ERROR, BUTTON_PRIMARY};

#[derive(Deserialize, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub(crate) enum Submit {
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

pub(crate) async fn get(Path { game_slug }: Path) -> impl IntoResponse {
    Html(
        Layout {
            main: markup::new! {
                div[
                    "hx-get"={crate::routes::partials::game_name::Path::new(game_slug.clone()).to_string()},
                    "hx-trigger"="revealed",
                    "hx-swap"="outerHTML"
                ] {
                    ."w-6"."h-6"."i-svg-spinners-gooey-balls-2"{}
                }
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
                            "hx-get"={crate::routes::partials::actor_kinds_table::Path::new(game_slug.clone()).to_string()},
                            "hx-trigger"="revealed","hx-swap"="outerHTML"
                        ] {
                            ."w-6"."h-6"."i-svg-spinners-gooey-balls-2"{}
                        }
                    }
                }
            },
        }
        .to_string(),
    )
}
