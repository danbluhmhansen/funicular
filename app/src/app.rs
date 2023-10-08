use crate::error_template::{AppError, ErrorTemplate};
use graphql_client::GraphQLQuery;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html lang="en" />
        <Stylesheet href="https://cdn.jsdelivr.net/npm/@unocss/reset/tailwind-compat.min.css" />
        <Stylesheet id="leptos" href="/site.css" />
        <Title text="Funicular" />
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <div class="min-h-screen dark:bg-slate-900 dark:text-white">
            <nav class="py-4">
                <ul class="flex flex-col sm:flex-row items-center justify-center gap-4">
                    <li><a href="/" class="hover:text-violet">"Home"</a></li>
                    <li><a href="/games" class="hover:text-violet">"Games"</a></li>
                </ul>
            </nav>
                <main class="container mx-auto flex flex-col items-center justify-center gap-4">
                    <Routes>
                        <Route path="/" view=Index />
                        <Route path="/games" view=Games />
                    </Routes>
                </main>
            </div>
        </Router>
    }
}

#[component]
fn Index() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button
            on:click=on_click
            class="
                bg-transparent hover:bg-violet-500 dark:hover:bg-violet-400
                border border-violet-600 dark:border-violet-300
                font-medium
                focus:outline-none
                px-4 py-2
                focus:ring-4 focus:ring-violet-400 dark:focus:ring-violet-500
                rounded
                text-sm text-center hover:text-white text-violet-600 dark:text-violet-300
            "
        >
            "Click Me: " {count}
        </button>
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "documents/games-query.graphql",
    response_derives = "Serialize"
)]
pub struct GamesQuery;

#[server]
pub async fn games_get() -> Result<Vec<games_query::GamesQueryGameCollectionEdges>, ServerFnError> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Profile", reqwest::header::HeaderValue::from_static("graphql"));

    let res = graphql_client::reqwest::post_graphql::<GamesQuery, _>(
        &reqwest::Client::builder().default_headers(headers).build().unwrap(),
        "http://localhost:3000/rpc/resolve",
        games_query::Variables,
    )
    .await?;

    Ok(res.data.map_or(vec![], |data| {
        data.game_collection
            .map_or(vec![], |game_collection| game_collection.edges)
    }))
}

#[component]
fn Games() -> impl IntoView {
    let games = create_resource(|| (), |_| async { games_get().await });
    let games_view = move || {
        games.and_then(|games| {
            games
                .iter()
                .map(|game| view! { <li>{game.node.name.clone()}</li> })
                .collect_view()
        })
    };

    view! {
        <h1>"Games"</h1>
        <Suspense fallback=move || view! { <p>"Loading games..."</p> }>
            <ul>
                {games_view}
            </ul>
        </Suspense>
    }
}
