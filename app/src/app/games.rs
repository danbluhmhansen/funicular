use postgrest::Postgrest;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, PartialEq, Deserialize)]
struct Game {
    id: Option<String>,
    name: Option<String>,
    created: Option<String>,
    description: Option<String>,
}

#[function_component]
pub fn Games() -> Html {
    let games = use_state(Vec::<Game>::new);
    {
        let games = games.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok(response) = Postgrest::new("http://localhost:3000")
                        .from("game")
                        .select("name")
                        .execute()
                        .await
                    {
                        if let Ok(fetched_games) = response.json().await {
                            games.set(fetched_games);
                        }
                    }
                });
                || ()
            },
            (),
        );
    }

    html! {
        <div class="mx-auto">
            <h1>{"Games"}</h1>
            {
                if games.len() > 0 {
                    html! {
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
                    }
                } else {
                    html! {"No games..."}
                }
        }
        </div>
    }
}
