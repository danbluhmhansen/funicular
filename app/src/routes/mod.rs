use axum::response::IntoResponse;
use axum_extra::routing::TypedPath;
use maud::html;

use crate::components::layout;

pub(crate) mod games;
pub(crate) mod partials;

pub(crate) async fn not_found() -> impl IntoResponse {
    layout(html! { (crate::components::not_found() ) })
}

#[derive(TypedPath)]
#[typed_path("/")]
pub(crate) struct IndexPath;

pub(crate) async fn index(_: IndexPath) -> impl IntoResponse {
    layout(html! { h1 class="text-xl font-bold" { "Funicular" } })
}
