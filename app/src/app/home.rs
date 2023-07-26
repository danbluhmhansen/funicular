use yew::prelude::*;

#[function_component]
pub fn Home() -> Html {
    html! {
        <div class={classes!("container", "mx-auto", "flex", "flex-col", "items-center")}>
            <h1>{"Home"}</h1>
        </div>
    }
}
