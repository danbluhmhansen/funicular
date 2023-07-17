use postgrest::Postgrest;
use serde::Deserialize;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
fn Header() -> Html {
    html! {
        <header class="mx-auto p-4">
            <nav>
                <ul class="flex flex-row justify-between space-x-8">
                    <li>
                        <Link<AppRoute> to={AppRoute::Home}>{ "Home" }</Link<AppRoute>>
                    </li>
                    <li>
                        <Link<AppRoute> to={AppRoute::Games}>{ "Games" }</Link<AppRoute>>
                    </li>
                </ul>
            </nav>
        </header>
    }
}

#[function_component]
fn Games() -> Html {
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

#[derive(Clone, Routable, PartialEq)]
enum AppRoute {
    #[at("/")]
    Home,
    #[at("/games")]
    Games,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! { <h1>{"Home"}</h1> },
        AppRoute::Games => html! { <Games /> },
        AppRoute::NotFound => html! { <h1>{"Not found."}</h1> },
    }
}

#[derive(Clone, PartialEq, Deserialize)]
struct Game {
    id: Option<String>,
    name: Option<String>,
    created: Option<String>,
    description: Option<String>,
}

#[function_component]
fn App() -> Html {
    html! {
        <div class="flex flex-col min-h-screen">
            <BrowserRouter>
                <Header />
                <Switch<AppRoute> render={switch} />
            </BrowserRouter>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
