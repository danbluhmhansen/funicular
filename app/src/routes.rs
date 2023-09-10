use axum::{http::header, response::IntoResponse};
use maud::html;

use crate::{components::Page, STYLE};

pub mod games;

pub async fn style() -> impl IntoResponse {
    (
        [
            (header::CACHE_CONTROL, "max-age=2592000"),
            (header::CONTENT_TYPE, "text/css"),
        ],
        STYLE,
    )
}

pub async fn index() -> impl IntoResponse {
    Page::new(html! {
        h1 class="text-lg" { "Hello, World!" }
        p class="p-2 text-red-500" {
            "Consequatur accusamus itaque illo ut saepe corporis voluptatem. Aut provident quasi voluptatem.
            Sunt non fuga officiis fugit aliquam numquam hic. Voluptatem ratione magni dolor ut."
        }
    })
    .render()
}

pub async fn not_found() -> impl IntoResponse {
    Page::new(html! { h1 class="text-xl font-bold" { "Not found" } }).render()
}
