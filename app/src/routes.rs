use axum::response::{Html, IntoResponse};
use axum_extra::routing::TypedPath;

use crate::components::{Layout, NotFound};

pub(crate) mod games;
pub(crate) mod partials;

pub(crate) async fn not_found() -> impl IntoResponse {
    Html(Layout { main: NotFound {} }.to_string())
}

#[derive(TypedPath)]
#[typed_path("/")]
pub(crate) struct IndexPath;

pub(crate) async fn index(_: IndexPath) -> impl IntoResponse {
    Html(
        Layout {
            main: markup::new! {
                h1."text-xl"."font-bold" { "Funicular" }
            },
        }
        .to_string(),
    )
}
