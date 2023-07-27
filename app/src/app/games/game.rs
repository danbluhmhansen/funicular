use yew::prelude::*;

#[function_component]
pub fn Game() -> Html {
    let fallback = html! {<span class={classes!("loading", "loading-infinity", "loading-lg")}></span>};
    html! {
        <div class={classes!("container", "mx-auto", "flex", "flex-col", "items-center")}>
            <h1>{"Game"}</h1>
            <Suspense {fallback}>
            </Suspense>
        </div>
    }
}
