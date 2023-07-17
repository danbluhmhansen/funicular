use yew::prelude::*;

#[function_component]
pub fn NotFound() -> Html {
    html! {
        <div class="mx-auto">
            <h1>{"Not found..."}</h1>
        </div>
    }
}
