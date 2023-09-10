use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect, Response},
};
use axum_typed_multipart::{TryFromField, TryFromMultipart, TypedMultipart};
use maud::html;
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use strum::Display;

use crate::{
    components::{Dialog, Page},
    routes::not_found,
    AppState, BUTTON_ERROR, BUTTON_PRIMARY, BUTTON_SUCCESS, BUTTON_WARNING, CAPTION, DIALOG, THEAD, TR,
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

async fn actor(game_slug: String, actor_kind_slug: String, actor_slug: String, pool: &Pool<Postgres>) -> Response {
    let game = sqlx::query!(
        "SELECT id, name, slug, description FROM game WHERE slug = $1;",
        game_slug
    )
    .fetch_one(pool)
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
    .fetch_one(pool)
    .await;

    if actor_kind.is_err() {
        return not_found().await.into_response();
    }

    let actor_kind = actor_kind.unwrap();

    let actor = sqlx::query!(
        r#"
            SELECT id, name, description
            FROM actor
            WHERE slug = $1 AND kind_id = $2;
        "#,
        actor_slug,
        actor_kind.id
    )
    .fetch_one(pool)
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
    .fetch_all(pool)
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
    .fetch_all(pool)
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
    .fetch_all(pool)
    .await;

    let gears = sqlx::query!(
        r#"
            SELECT gear.name
            FROM gear
            JOIN gear_kind ON gear_kind.id = gear.kind_id
            JOIN game ON game.id = gear_kind.game_id
            WHERE game.slug = $1;
        "#,
        game_slug
    )
    .fetch_all(pool)
    .await;

    let traits = sqlx::query!(
        r#"
            SELECT trait.name
            FROM trait
            JOIN game on game.id = trait.game_id
            WHERE game.slug = $1;
        "#,
        game_slug
    )
    .fetch_all(pool)
    .await;

    Page::new(html! {
        ol class="flex flex-row" {
            li {
                a href={"/games/" (game_slug)} class="hover:text-violet-500" { (game.name) }
            }
            li class="flex flex-row justify-center items-center" {
              div class="i-tabler-chevron-right";
            }
            li {
                a href={"/games/" (game_slug) "/actors/" (actor_kind_slug)} class="hover:text-violet-500" {
                    (actor_kind.name)
                }
            }
        }
        div class="flex flex-row gap-2 justify-center items-center" {
            h1 class="text-xl font-bold" { (actor.name) }
            a href={"#" (Submit::Edit)} class=(BUTTON_WARNING) { span class="w-4 h-4 i-tabler-pencil" {} }
        }
        @if let Some(description) = &actor.description { p { (description) } }
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
        div class="overflow-x-auto relative shadow-md rounded" {
            table class="w-full" {
                caption class=(CAPTION) {
                    a href={"#" (Submit::GearAdd)} class=(BUTTON_PRIMARY) { span class="w-4 h-4 i-tabler-plus" {} }
                    button type="submit" name="submit" value=(Submit::GearRemove) class=(BUTTON_ERROR) {
                        span class="w-4 h-4 i-tabler-trash" {}
                    }
                }
                thead class=(THEAD) {
                    tr {
                        th class="p-3 text-center" {
                            input type="checkbox" name="slugs_all" value="true" class="bg-transparent";
                        }
                        th class="py-3 px-6 text-left" { "Name" }
                        th class="py-3 px-6 text-left" { div class="w-4 h-4 i-tabler-hash"; }
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
        h2 class="text-xl font-bold" { "Traits" }
        div class="overflow-x-auto relative shadow-md rounded" {
            table class="w-full" {
                caption class=(CAPTION) {
                    a href={"#" (Submit::TraitAdd)} class=(BUTTON_PRIMARY) { span class="w-4 h-4 i-tabler-plus" {} }
                    button type="submit" name="submit" value=(Submit::TraitRemove) class=(BUTTON_ERROR) {
                        span class="w-4 h-4 i-tabler-trash" {}
                    }
                }
                thead class=(THEAD) {
                    tr {
                        th class="p-3 text-center" {
                            input type="checkbox" name="slugs_all" value="true" class="bg-transparent";
                        }
                        th class="py-3 px-6 text-left" { "Name" }
                        th class="py-3 px-6 text-left" { div class="w-4 h-4 i-tabler-hash"; }
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
    .dialog(
        Dialog::new(html! {
            form method="post" enctype="multipart/form-data" class="flex flex-col gap-4 justify-center" {
                input type="hidden" name="actor_id" value=(actor.id);
                input
                    type="text"
                    name="name"
                    value=(actor.name)
                    placeholder="Name"
                    required
                    autofocus
                    class="bg-transparent rounded invalid:border-red";
                textarea
                    name="description"
                    value=[&actor.description]
                    placeholder="Description"
                    class="rounded invalid:border-red dark:bg-slate-900" {
                    @if let Some(description) = &actor.description { (description) }
                }
                div class="flex justify-between" {
                    button type="submit" name="submit" value=(Submit::Edit) class=(BUTTON_SUCCESS) {
                        span class="w-4 h-4 i-tabler-check" {}
                    }
                }
            }
        })
        .id(Submit::Edit)
        .title(&format!("Edit {}", actor_kind.name))
    )
    .dialog(
        Dialog::new(html! {
            form method="post" enctype="multipart/form-data" class="flex flex-col gap-4 justify-center" {
                input type="hidden" name="actor_id" value=(actor.id);
                div class="overflow-x-auto relative shadow-md rounded" {
                    table class="w-full" {
                        caption class=(CAPTION) {
                            a href={"#" (Submit::TraitAdd)} class=(BUTTON_PRIMARY) { span class="w-4 h-4 i-tabler-plus" {} }
                            button type="submit" name="submit" value=(Submit::TraitRemove) class=(BUTTON_ERROR) {
                                span class="w-4 h-4 i-tabler-trash" {}
                            }
                        }
                        thead class=(THEAD) {
                            tr {
                                th class="p-3 text-center" {
                                    input type="checkbox" name="slugs_all" value="true" class="bg-transparent";
                                }
                                th class="py-3 px-6 text-left" { "Name" }
                            }
                        }
                        tbody {
                            @match gears {
                                Ok(gears) => {
                                    @for gear in gears {
                                        tr class=(TR) {
                                            td class="p-3 text-center" {
                                                input
                                                    type="checkbox"
                                                    name="slugs"
                                                    class="bg-transparent";
                                            }
                                            td class="py-3 px-6" { (gear.name) }
                                        }
                                    }
                                }
                                Err(_) => { p { "No gear available..." } }
                            }
                        }
                    }
                }
                div class="flex justify-between" {
                    button type="submit" name="submit" value=(Submit::GearAdd) class=(BUTTON_SUCCESS) {
                        span class="w-4 h-4 i-tabler-check" {}
                    }
                }
            }
        })
        .id(Submit::GearAdd)
        .title("Add Gear")
    )
    .dialog(
        Dialog::new(html! {
            form method="post" enctype="multipart/form-data" class="flex flex-col gap-4 justify-center" {
                input type="hidden" name="actor_id" value=(actor.id);
                div class="overflow-x-auto relative shadow-md rounded" {
                    table class="w-full" {
                        caption class=(CAPTION) {
                            a href={"#" (Submit::TraitAdd)} class=(BUTTON_PRIMARY) { span class="w-4 h-4 i-tabler-plus" {} }
                            button type="submit" name="submit" value=(Submit::TraitRemove) class=(BUTTON_ERROR) {
                                span class="w-4 h-4 i-tabler-trash" {}
                            }
                        }
                        thead class=(THEAD) {
                            tr {
                                th class="p-3 text-center" {
                                    input type="checkbox" name="slugs_all" value="true" class="bg-transparent";
                                }
                                th class="py-3 px-6 text-left" { "Name" }
                            }
                        }
                        tbody {
                            @match traits {
                                Ok(traits) => {
                                    @for t in traits {
                                        tr class=(TR) {
                                            td class="p-3 text-center" {
                                                input
                                                    type="checkbox"
                                                    name="slugs"
                                                    class="bg-transparent";
                                            }
                                            td class="py-3 px-6" { (t.name) }
                                        }
                                    }
                                }
                                Err(_) => { p { "No trait available..." } }
                            }
                        }
                    }
                }
                div class="flex justify-between" {
                    button type="submit" name="submit" value=(Submit::TraitAdd) class=(BUTTON_SUCCESS) {
                        span class="w-4 h-4 i-tabler-check" {}
                    }
                }
            }
        })
        .id(Submit::TraitAdd)
        .title("Add Traits")
    )
    .render()
    .into_response()
}

pub async fn actor_get(
    Path(ActorPath {
        game_slug,
        actor_kind_slug,
        actor_slug,
    }): Path<ActorPath>,
    State(state): State<Arc<AppState>>,
) -> Response {
    actor(game_slug, actor_kind_slug, actor_slug, &state.pool).await
}

#[derive(TryFromMultipart)]
pub struct Payload {
    pub submit: Submit,
    pub actor_id: Option<uuid::Uuid>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub slugs_all: Option<bool>,
    pub slugs: Vec<String>,
}

pub async fn actor_post(
    Path(ActorPath {
        game_slug,
        actor_kind_slug,
        actor_slug,
    }): Path<ActorPath>,
    State(state): State<Arc<AppState>>,
    TypedMultipart(form): TypedMultipart<Payload>,
) -> Response {
    match form.submit {
        Submit::Edit => {
            let actor_res = sqlx::query!(
                "UPDATE actor SET name = $1, description = $2 WHERE id = $3 RETURNING slug;",
                form.name,
                form.description,
                form.actor_id
            )
            .fetch_one(&state.pool)
            .await;

            if actor_res.is_err() {
                todo!()
            }

            let new_slug = actor_res.unwrap().slug;

            if actor_slug != new_slug {
                Redirect::to(&format!("/games/{game_slug}/actors/{actor_kind_slug}/{new_slug}")).into_response()
            } else {
                actor(game_slug, actor_kind_slug, actor_slug, &state.pool)
                    .await
                    .into_response()
            }
        }
        Submit::GearAdd => actor(game_slug, actor_kind_slug, actor_slug, &state.pool).await,
        Submit::GearRemove => actor(game_slug, actor_kind_slug, actor_slug, &state.pool).await,
        Submit::TraitAdd => actor(game_slug, actor_kind_slug, actor_slug, &state.pool).await,
        Submit::TraitRemove => actor(game_slug, actor_kind_slug, actor_slug, &state.pool).await,
    }
}
