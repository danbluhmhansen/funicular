use axum::response::IntoResponse;
use maud::html;

use crate::components::page;

pub async fn index() -> impl IntoResponse {
    page(
        html! {
            h1 class="text-lg" { "Hello, World!" }
            p class="p-2 text-red-500" {
                "Consequatur accusamus itaque illo ut saepe corporis voluptatem. Aut provident quasi voluptatem.
                Sunt non fuga officiis fugit aliquam numquam hic. Voluptatem ratione magni dolor ut."
            }
        },
        None,
    )
}
