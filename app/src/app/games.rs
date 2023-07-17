use postgrest::Postgrest;
use serde::Deserialize;
use yew::{
    prelude::*,
    suspense::{use_future, SuspensionResult},
};

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
        .select("name")
        .execute()
        .await?
        .json::<Vec<Game>>()
        .await
}

#[hook]
fn use_games() -> SuspensionResult<Vec<Game>> {
    // TODO: handle unwrap
    Ok((*(use_future(fetch_games)?.as_ref().unwrap())).clone())
}

#[function_component]
fn Content() -> HtmlResult {
    let games = use_games()?;
    if games.len() > 0 {
        Ok(html! {
            <table class="table-auto border-collapse mx-auto">
                <thead>
                    <tr class="px-4 py-2">
                        <th>{"Name"}</th>
                    </tr>
                </thead>
                <tbody>
                    {games.iter().map(|g|
                        html! {
                            <tr class="px-4 py-2">
                                <td>
                                    {g.name.clone()}
                                </td>
                            </tr>
                        }
                    ).collect::<Html>()}
                </tbody>
            </table>
        })
    } else {
        Ok(html! {"No games..."})
    }
}

#[function_component]
pub fn Games() -> Html {
    let fallback = html! {"fallback"};
    html! {
        <div class="mx-auto">
            <h1>{"Games"}</h1>
            <Suspense {fallback}>
                <Content />
            </Suspense>
        </div>
    }
}
