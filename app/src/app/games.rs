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
    let games = use_games()?;
    if !games.is_empty() {
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
                            <tr key={g.id.as_ref().unwrap().clone()} class="px-4 py-2">
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
