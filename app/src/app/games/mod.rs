pub mod game;

use postgrest::Postgrest;
use serde::Deserialize;
use yew::{
    prelude::*,
    suspense::{use_future, SuspensionResult},
};
use yew_router::prelude::*;

use crate::app::GamesRoute;

#[derive(Clone, PartialEq, Deserialize)]
struct Game {
    id: Option<String>,
    name: Option<String>,
    created: Option<String>,
    description: Option<String>,
}

async fn fetch_games() -> Result<Vec<Game>, reqwest::Error> {
    Postgrest::new("http://localhost:3000")
        .from("game")
        .select("id,name")
        .execute()
        .await?
        .json::<Vec<Game>>()
        .await
}

#[hook]
fn use_games() -> SuspensionResult<Vec<Game>> {
    Ok(use_future(fetch_games)?.as_deref().unwrap().to_vec())
}

#[function_component]
fn Content() -> HtmlResult {
    let games = use_games()?
        .into_iter()
        .filter_map(|g| g.id.zip(g.name))
        .collect::<Vec<_>>();
    if games.is_empty() {
        return Ok(html! {"No games..."});
    }
    Ok(html! {
        <table class={classes!("table")}>
            <thead>
                <tr>
                    <th>{"Name"}</th>
                </tr>
            </thead>
            <tbody>
                {games.into_iter().map(|g|
                    html! {
                        <Link<GamesRoute> to={GamesRoute::Game { name: g.1.clone() }}>
                            <tr key={g.0}><td>{g.1}</td></tr>
                        </Link<GamesRoute>>
                    }
                ).collect::<Html>()}
            </tbody>
        </table>
    })
}

#[function_component]
pub fn Games() -> Html {
    let fallback = html! {<span class={classes!("loading", "loading-infinity", "loading-lg")}></span>};
    html! {
        <div class={classes!("container", "mx-auto", "flex", "flex-col", "items-center")}>
            <h1>{"Games"}</h1>
            <Suspense {fallback}>
                <Content />
            </Suspense>
        </div>
    }
}
