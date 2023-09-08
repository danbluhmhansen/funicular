use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use maud::html;
use serde::Deserialize;

use crate::{components::Page, AppState, BUTTON_ERROR, BUTTON_PRIMARY, BUTTON_WARNING};

#[derive(Deserialize)]
pub struct ActorPath {
    pub game_slug: String,
    pub actor_kind_slug: String,
    pub actor_slug: String,
}

pub async fn actor(
    Path(ActorPath {
        game_slug,
        actor_kind_slug,
        actor_slug,
    }): Path<ActorPath>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let game = sqlx::query!(
        "SELECT id, name, slug, description FROM game WHERE slug = $1;",
        game_slug
    )
    .fetch_one(&state.pool)
    .await
    .unwrap();

    let actor_kind = sqlx::query!(
        "SELECT id, name FROM actor_kind WHERE slug = $1 AND game_id = $2",
        actor_kind_slug,
        game.id
    )
    .fetch_one(&state.pool)
    .await
    .unwrap();

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
    .await
    .unwrap();

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
    .await
    .unwrap();

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
    .await
    .unwrap();

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
    .await
    .unwrap();

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
            a href="#edit" class=(BUTTON_WARNING) { span class="w-4 h-4 i-tabler-pencil"; }
        }
        div class="overflow-x-auto relative shadow-md rounded" {
            table class="w-full" {
                thead class="text-xs text-gray-700 uppercase dark:text-gray-400 bg-slate-50 dark:bg-slate-700" {
                    tr { @for skill in &skills { th class="py-3 px-6 text-left" { (skill.name) } } }
                }
                tbody {
                    tr class="bg-white border-b last:border-0 dark:bg-slate-800 dark:border-slate-700" {
                        @for skill in skills {
                            td class="py-3 px-6 text-center" { (skill.value.unwrap()) }
                        }
                    }
                }
            }
        }
        h2 class="text-xl font-bold" { "Gear" }
        div class="overflow-x-auto relative shadow-md rounded w-96" {
            table class="w-full" {
                caption class="p-3 space-x-2 bg-white dark:bg-slate-800" {
                    a href="#add" class=(BUTTON_PRIMARY) { span class="w-4 h-4 i-tabler-plus"; }
                    button type="submit" name="submit" value="remove" class=(BUTTON_ERROR) {
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
                    @for gear in actor_gears {
                        tr class="bg-white border-b last:border-0 dark:bg-slate-800 dark:border-slate-700" {
                            td class="p-3 text-center" {
                                input
                                    type="checkbox"
                                    name="slugs"
                                    value=(gear.slug)
                                    class="bg-transparent";
                            }
                            td class="py-3 px-6" { (gear.name) }
                            td class="py-3 px-6" { (gear.amount.unwrap()) }
                        }
                    }
                }
            }
        }
        h2 class="text-xl font-bold" { "traits" }
        div class="overflow-x-auto relative shadow-md rounded w-96" {
            table class="w-full" {
                caption class="p-3 space-x-2 bg-white dark:bg-slate-800" {
                    a href="#add" class=(BUTTON_PRIMARY) { span class="w-4 h-4 i-tabler-plus"; }
                    button type="submit" name="submit" value="remove" class=(BUTTON_ERROR) {
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
                    @for t in actor_traits {
                        tr class="bg-white border-b last:border-0 dark:bg-slate-800 dark:border-slate-700" {
                            td class="p-3 text-center" {
                                input
                                    type="checkbox"
                                    name="slugs"
                                    value=(t.slug)
                                    class="bg-transparent";
                            }
                            td class="py-3 px-6" { (t.name) }
                            td class="py-3 px-6" { (t.amount.unwrap()) }
                        }
                    }
                }
            }
        }
    })
    .build()
}
