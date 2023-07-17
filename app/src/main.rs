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
fn App() -> Html {
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
        <div class="flex flex-col min-h-screen">
            <header class="mx-auto p-4">
                <nav>
                    <ul class="flex flex-row justify-between space-x-8">
                        <li>
                            <a href="/">{"Home"}</a>
                        </li>
                        <li>
                          <a href="/games">{"Games"}</a>
                        </li>
                    </ul>
                </nav>
            </header>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
