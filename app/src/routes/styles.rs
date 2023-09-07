use axum::http::header;
use axum::response::IntoResponse;

use crate::STYLE;

pub async fn style() -> impl IntoResponse {
    (
        [
            (header::CACHE_CONTROL, "max-age=2592000"),
            (header::CONTENT_TYPE, "text/css"),
        ],
        STYLE,
    )
}
