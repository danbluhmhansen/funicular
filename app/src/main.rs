use std::{error::Error, sync::Arc, time::Duration};

use axum::{
    extract::State,
    response::{Html, IntoResponse},
    Router, Server,
};
use axum_extra::{
    extract::Form,
    routing::{RouterExt, TypedPath},
};
use const_format::formatcp;
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tower_livereload::LiveReloadLayer;

markup::define! {
    Layout<Main: markup::Render>(main: Main) {
        @markup::doctype()
        html {
            head {
                meta[charset="utf-8"];
                meta[name="viewport",contet="width=device-width,initial-scale=1.0"];
                title { "Funicular" }
                link[rel="icon",type="image/svg+xml",href="data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxZW0iIGhlaWdodD0iMWVtIiB2aWV3Qm94PSIwIDAgMjQgMjQiPjxwYXRoIGZpbGw9Im5vbmUiIHN0cm9rZT0iY3VycmVudENvbG9yIiBkPSJNNy40NzggMTguMTQ5YTEuNSAxLjUgMCAwIDEtMi45NTQuNTJtMTEuOTk5LTIuMjVhMS41IDEuNSAwIDAgMCAyLjk1NC0uNTJNOCAxMS43NThWNC42MzZtOCA1LjY0OFYzLjE4Mm02Ljk3IDYuMjNjLjAxOS0uNDc3LjAzLS45OC4wMy0xLjUwM0MyMyA0LjQxIDIyLjUgMiAyMi41IDJsLTIxIDMuODE4UzEgOC40MSAxIDExLjkxYzAgLjUyMy4wMTEgMS4wMjIuMDMgMS40OTJtMjEuOTQtMy45OUMyMi44NjIgMTIuMTI3IDIyLjUgMTQgMjIuNSAxNGwtMjEgMy44MThzLS4zNjItMS43NDMtLjQ3LTQuNDE3bTIxLjk0LTMuOTljLTEwLjY1Ni45NzMtMjEuMzAyIDMuODE4LTIxLjk0IDMuOTlNMjMgMTlMMSAyMyIvPjwvc3ZnPg=="];
                style {
                    "[un-cloak]{display:table-column;}@media(prefers-color-scheme:dark){[un-cloak]{background-color:#0f172a;}}"
                }
                link[rel="stylesheet",href="https://cdn.jsdelivr.net/npm/@unocss/reset/tailwind-compat.min.css"];
                script[type="module"] {
                    "import initUnocssRuntime from 'https://cdn.jsdelivr.net/npm/@unocss/runtime@0.56.5/+esm';import presetUno from 'https://cdn.jsdelivr.net/npm/@unocss/preset-uno@0.56.5/+esm';import { presetForms } from 'https://cdn.jsdelivr.net/npm/@julr/unocss-preset-forms@0.0.5/+esm';import presetIcons from 'https://cdn.jsdelivr.net/npm/@unocss/preset-icons@0.56.5/+esm';initUnocssRuntime({defaults:{presets:[presetUno({dark:'media'}),presetForms(),presetIcons({cdn:'https://esm.sh/'})]}});"
                }
                script[src="https://unpkg.com/htmx.org@1.9.6"]{}
            }
            body."dark:text-white"."dark:bg-slate-900"["un-cloak","hx-boost"="true"] {
                nav."py-4" {
                    ul.flex."flex-col"."gap-4"."justify-center"."items-center"."sm:flex-row" {
                        li { a."hover:text-violet"[href="/"] { "Home" } }
                        li { a."hover:text-violet"[href="/games"] { "Games" } }
                    }
                }
                main.container.flex."flex-col"."gap-4"."justify-center"."items-center"."mx-auto" {
                    @main
                }
            }
        }
    }
}

async fn not_found() -> impl IntoResponse {
    Html(
        Layout {
            main: markup::new! {
                h1 { "Not found" }
            },
        }
        .to_string(),
    )
}

const BUTTON: &str = "inline-flex items-center py-2 px-4 text-sm font-medium text-center bg-transparent rounded border hover:text-white focus:ring-4 focus:outline-none";
const BUTTON_PRIMARY: &str = formatcp!("{BUTTON} {}", " text-violet-600 border-violet-600 dark:text-violet-300 dark:border-violet-300 hover:bg-violet-500 focus:ring-violet-400 dark:hover:bg-violet-400 dark:focus:ring-violet-500");
// const BUTTON_SUCCESS: &str = formatcp!("{BUTTON} {}", " text-green-600 border-green-600 dark:text-green-300 dark:border-green-300 hover:bg-green-500 focus:ring-green-400 dark:hover:bg-green-400 dark:focus:ring-green-500");
// const BUTTON_WARNING: &str = formatcp!("{BUTTON} {}", " text-amber-600 border-amber-600 dark:text-amber-300 dark:border-amber-300 hover:bg-amber-500 focus:ring-amber-400 dark:hover:bg-amber-400 dark:focus:ring-amber-500");
const BUTTON_ERROR: &str = formatcp!("{BUTTON} {}", " text-red-600 border-red-600 dark:text-red-300 dark:border-red-300 hover:bg-red-500 focus:ring-red-400 dark:hover:bg-red-400 dark:focus:ring-red-500");

#[derive(TypedPath)]
#[typed_path("/")]
struct IndexPath;

async fn index(_: IndexPath) -> impl IntoResponse {
    Html(
        Layout {
            main: markup::new! {
                h1 { "Funicular" }
            },
        }
        .to_string(),
    )
}

#[derive(TypedPath)]
#[typed_path("/games")]
struct GamesPath;

async fn games_get(_: GamesPath) -> impl IntoResponse {
    Html(
        Layout {
            main: markup::new! {
                h1 { "Games" }
                ."overflow-x-auto".relative.rounded."shadow-md" {
                    form[method="post"] {
                        .flex."flex-row"."gap-2"."justify-center"."p-3"."bg-white"."dark:bg-slate-800" {
                            a[href="#add",class={BUTTON_PRIMARY}] { ."w-4"."h-4"."i-tabler-plus"{} }
                            button[type="submit",name="submit",value="remove",class={BUTTON_ERROR}] {
                                ."w-4"."h-4"."i-tabler-trash"{}
                            }
                        }
                        div["hx-get"="/partials/games-table","hx-trigger"="revealed","hx-swap"="outerHTML"] {
                            "Loading..."
                        }
                    }
                }
                dialog.hidden."inset-0"."z-10"."justify-center"."items-center"."w-full"."h-full"."target:flex"."bg-black/50"."backdrop-blur-sm"[id="add"] {
                    .flex."z-10"."flex-col"."gap-4"."p-4"."max-w-sm"."bg-white".rounded.border."dark:text-white"."dark:bg-slate-900" {
                        div {
                            a."float-right"."w-4"."h-4"."i-tabler-x"[href="#!"]{}
                            h2."text-xl" { "Add Game" }
                        }
                        form.flex."flex-col"."gap-4"."justify-center"[method="post"] {
                            input."bg-transparent".rounded."invalid:border-red"[type="text",name="name",placeholder="Name",required,autofocus];
                            textarea."bg-transparent".rounded."invalid:border-red"[name="description",placeholder="Description"]{}
                            .flex."justify-between" {
                                button[type="submit",name="submit",value="add",class={BUTTON_PRIMARY}] {
                                    ."w-4"."h-4"."i-tabler-check"{}
                                }
                            }
                        }
                    }
                    a.fixed."inset-0"[href="#!"]{}
                }
            },
        }
        .to_string(),
    )
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum GamesFormSubmit {
    Add,
    Remove,
}

#[derive(Deserialize)]
struct GamesForm {
    submit: GamesFormSubmit,
    name: Option<String>,
    description: Option<String>,
    #[serde(default)]
    slugs: Vec<String>,
}

async fn games_post(
    path: GamesPath,
    State(state): State<Arc<AppState>>,
    Form(form): Form<GamesForm>,
) -> impl IntoResponse {
    match form.submit {
        GamesFormSubmit::Add => {
            let res = sqlx::query!(
                "INSERT INTO game (name, description) VALUES ($1, $2);",
                form.name,
                form.description
            )
            .execute(&state.pool)
            .await;
        }
        GamesFormSubmit::Remove => {
            let res = sqlx::query!("DELETE FROM game WHERE slug = ANY($1);", &form.slugs)
                .execute(&state.pool)
                .await;
        }
    }

    games_get(path).await
}

#[derive(TypedPath)]
#[typed_path("/partials/games-table")]
struct GamesTablePath;

async fn games_table(_: GamesTablePath, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let games = sqlx::query!("SELECT name, slug FROM game;")
        .fetch_all(&state.pool)
        .await
        .map_or(vec![], |games| games);
    Html(
        markup::new! {
            table."w-full" {
                thead."text-xs"."text-gray-700".uppercase."dark:text-gray-400"."bg-slate-50"."dark:bg-slate-700" {
                    tr {
                        th."p-3"."text-center" { input."bg-transparent"[type="checkbox"]; }
                        th."p-3"."text-center" { "Name" }
                    }
                }
                tbody {
                    @for game in games.iter() {
                        tr."bg-white"."border-b"."last:border-0"."dark:bg-slate-800"."dark:border-slate-700" {
                            td."p-3"."text-center" {
                                input."bg-transparent"[type="checkbox",name="slugs",value={game.slug.clone()}];
                            }
                            td."p-3"."text-center" {
                                a."hover:text-violet"[href={format!("/games/{}", game.slug)}] { @game.name }
                            }
                        }
                    }
                }
            }
        }
        .to_string(),
    )
}

pub struct AppState {
    pool: Pool<Postgres>,
}

impl AppState {
    fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let shared_state = Arc::new(AppState::new(
        PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(
                &std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost:28816/funicular".to_string()),
            )
            .await?,
    ));

    let app = Router::new()
        .typed_get(index)
        .typed_get(games_get)
        .typed_post(games_post)
        .typed_get(games_table)
        .fallback(not_found)
        .with_state(shared_state);

    #[cfg(debug_assertions)]
    let app = app.layer(LiveReloadLayer::new());

    Server::bind(&([0, 0, 0, 0], 1111).into())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
