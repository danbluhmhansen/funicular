use std::sync::Arc;

use axum::response::{Html, IntoResponse};
use axum_extra::routing::TypedPath;
use serde::Deserialize;

use crate::components::Layout;

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
            },
        }
        .to_string(),
    )
}
