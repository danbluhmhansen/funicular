use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use axum_typed_multipart::TryFromField;
use maud::html;
use serde::Deserialize;
use strum::Display;

use crate::{
    components::Page, routes::not_found, AppState, BUTTON_ERROR, BUTTON_PRIMARY, BUTTON_WARNING, CAPTION, THEAD, TR,
};

#[derive(Deserialize)]
pub struct ActorPath {
    pub game_slug: String,
    pub actor_kind_slug: String,
    pub actor_slug: String,
}

#[derive(Display, TryFromField)]
#[strum(serialize_all = "snake_case")]
#[try_from_field(rename_all = "snake_case")]
pub enum Submit {
    Edit,
    GearAdd,
    GearRemove,
    TraitAdd,
    TraitRemove,
}

pub async fn actor(
    Path(ActorPath {
        game_slug,
        actor_kind_slug,
        actor_slug,
    }): Path<ActorPath>,
    State(state): State<Arc<AppState>>,
) -> Response {
    let game = sqlx::query!(
        "SELECT id, name, slug, description FROM game WHERE slug = $1;",
        game_slug
    )
    .fetch_one(&state.pool)
    .await;

    if game.is_err() {
        return not_found().await.into_response();
    }

    let game = game.unwrap();

    let actor_kind = sqlx::query!(
        "SELECT id, name FROM actor_kind WHERE slug = $1 AND game_id = $2",
        actor_kind_slug,
        game.id
    )
    .fetch_one(&state.pool)
    .await;

    if actor_kind.is_err() {
        return not_found().await.into_response();
    }

    let actor_kind = actor_kind.unwrap();

    let actor = sqlx::query!(
        r#"
            SELECT id, name
            FROM actor
            WHERE slug = $1 AND kind_id = $2;
        "#,
        actor_slug,
        actor_kind.id
    )
    .fetch_one(&state.pool)
    .await;

    if actor.is_err() {
        return not_found().await.into_response();
    }

    let actor = actor.unwrap();

    let skills = sqlx::query!(
        r#"
            SELECT skill.name, actor_num_skill.value
            FROM actor_num_skill
            JOIN actor ON actor.id = actor_num_skill.actor_id
            JOIN actor_kind ON actor_kind.id = actor.kind_id
            JOIN skill ON skill.id = actor_num_skill.skill_id
            WHERE actor.id = $1;
        "#,
        actor.id
    )
    .fetch_all(&state.pool)
    .await;

    let actor_gears = sqlx::query!(
        r#"
            SELECT gear.name, gear.slug, actor_gear.amount
            FROM actor_gear
            JOIN actor on actor.id = actor_gear.actor_id
            JOIN gear on gear.id = actor_gear.gear_id
            WHERE actor.id = $1;
        "#,
        actor.id
    )
    .fetch_all(&state.pool)
    .await;

    let actor_traits = sqlx::query!(
        r#"
            SELECT trait.name, trait.slug, actor_trait.amount
            FROM actor_trait
            JOIN actor ON actor.id = actor_trait.actor_id
            JOIN trait ON trait.id = actor_trait.trait_id
            WHERE actor.id = $1;
        "#,
        actor.id
    )
    .fetch_all(&state.pool)
    .await;

    Page::new(html! {
        ol class="flex flex-row" {
            li {
                a href={"/games/" (game_slug)} class="hover:text-violet-500" { (game.name) }
            }
            li class="flex flex-row justify-center items-center" {
              div class="i-tabler-chevron-right" {}
            }
            li {
                a href={"/games/" (game_slug) "/actors/" (actor_kind_slug)} class="hover:text-violet-500" {
                    (actor_kind.name)
                }
            }
        }
        div class="flex flex-row gap-2 justify-center items-center" {
            h1 class="text-xl font-bold" { (actor.name) }
            a href={"#" (Submit::Edit)} class=(BUTTON_WARNING) { span class="w-4 h-4 i-tabler-pencil"; }
        }
        @match skills {
            Ok(skills) => {
                div class="overflow-x-auto relative shadow-md rounded" {
                    table class="w-full" {
                        thead class=(THEAD) {
                            tr { @for skill in &skills { th class="py-3 px-6 text-left" { (skill.name) } } }
                        }
                        tbody {
                            tr class=(TR) {
                                @for skill in skills {
                                    td class="py-3 px-6 text-center" {
                                        @match skill.value {
                                            Some(value) => (value),
                                            None => "0",
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(_) => { "No skills..." }
        }
        h2 class="text-xl font-bold" { "Gear" }
        div class="overflow-x-auto relative shadow-md rounded w-96" {
            table class="w-full" {
                caption class=(CAPTION) {
                    a href={"#" (Submit::GearAdd)} class=(BUTTON_PRIMARY) { span class="w-4 h-4 i-tabler-plus"; }
                    button type="submit" name="submit" value=(Submit::GearRemove) class=(BUTTON_ERROR) {
                        span class="w-4 h-4 i-tabler-trash";
                    }
                }
                thead class="
                        text-xs
                        text-gray-700
                        uppercase
                        dark:text-gray-400
                        bg-slate-50
                        dark:bg-slate-700
                    " {
                    tr {
                        th class="p-3 text-center" {
                            input type="checkbox" name="slugs_all" value="true" class="bg-transparent";
                        }
                        th class="py-3 px-6 text-left" { "Name" }
                        th class="py-3 px-6 text-left" { "Amount" }
                    }
                }
                tbody {
                    @match actor_gears {
                        Ok(actor_gears) => {
                            @for gear in actor_gears {
                                tr class=(TR) {
                                    td class="p-3 text-center" {
                                        input
                                            type="checkbox"
                                            name="slugs"
                                            value=(gear.slug)
                                            class="bg-transparent";
                                    }
                                    td class="py-3 px-6" { (gear.name) }
                                    td class="py-3 px-6" {
                                        @match gear.amount {
                                            Some(amount) => (amount),
                                            None => "0",
                                        }
                                    }
                                }
                            }
                        }
                        Err(_) => { "No gear..." }
                    }
                }
            }
        }
        h2 class="text-xl font-bold" { "traits" }
        div class="overflow-x-auto relative shadow-md rounded w-96" {
            table class="w-full" {
                caption class=(CAPTION) {
                    a href={"#" (Submit::TraitAdd)} class=(BUTTON_PRIMARY) { span class="w-4 h-4 i-tabler-plus"; }
                    button type="submit" name="submit" value=(Submit::TraitRemove) class=(BUTTON_ERROR) {
                        span class="w-4 h-4 i-tabler-trash";
                    }
                }
                thead class="
                        text-xs
                        text-gray-700
                        uppercase
                        dark:text-gray-400
                        bg-slate-50
                        dark:bg-slate-700
                    " {
                    tr {
                        th class="p-3 text-center" {
                            input type="checkbox" name="slugs_all" value="true" class="bg-transparent";
                        }
                        th class="py-3 px-6 text-left" { "Name" }
                        th class="py-3 px-6 text-left" { "Amount" }
                    }
                }
                tbody {
                    @match actor_traits {
                        Ok(actor_traits) => {
                            @for t in actor_traits {
                                tr class=(TR) {
                                    td class="p-3 text-center" {
                                        input
                                            type="checkbox"
                                            name="slugs"
                                            value=(t.slug)
                                            class="bg-transparent";
                                    }
                                    td class="py-3 px-6" { (t.name) }
                                    td class="py-3 px-6" {
                                        @match t.amount {
                                            Some(amount) => (amount),
                                            None => "0",
                                        }
                                    }
                                }
                            }
                        }
                        Err(_) => { "No traits..." }
                    }
                }
            }
        }
    })
    .build()
    .into_response()
}
