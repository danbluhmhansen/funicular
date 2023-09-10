use axum::response::IntoResponse;
use maud::{html, Render};

use crate::components::Page;

pub mod games;

pub async fn index() -> impl IntoResponse {
    Page::new(html! {
        h1 class="text-lg" { "Hello, World!" }
        p class="p-2 text-red-500" {
            "Consequatur accusamus itaque illo ut saepe corporis voluptatem. Aut provident quasi voluptatem.
            Sunt non fuga officiis fugit aliquam numquam hic. Voluptatem ratione magni dolor ut."
        }
    })
}

pub async fn not_found() -> impl IntoResponse {
    Page::new(html! { h1 class="text-xl font-bold" { "Not found" } }).render()
}
