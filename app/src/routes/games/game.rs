use axum_extra::routing::TypedPath;
use serde::Deserialize;

#[derive(Deserialize, TypedPath)]
#[typed_path("/games/:game_slug")]
pub(crate) struct Path {
    game_slug: String,
}

impl Path {
    pub(crate) fn new(game_slug: String) -> Self {
        Self { game_slug }
    }
}
