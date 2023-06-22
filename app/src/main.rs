use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    pub children: Children,
}

#[function_component]
fn Button(props: &ButtonProps) -> Html {
    let class = classes!(
        "px-4",
        "py-2",
        "rounded-md",
        "dark:text-white",
        "bg-cyan-500",
        "hover:bg-cyan-600",
        "active:bg-cyan-700",
        "focus:outline-none",
        "focus:ring",
        "focus:ring-cyan-300",
    );

    html! {
        <button onclick={&props.onclick} {class}>
            { for props.children.iter() }
        </button>
    }
}

#[derive(Deserialize)]
struct Character {
    id: Option<String>,
    name: Option<String>,
}

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    let characters: Vec<Character> = vec![];
    let characters = use_state(|| characters);
    {
        let characters = characters.clone();
        use_effect(move || {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(response) = Request::get("http://localhost:3000/character").send().await {
                    if let Ok(fetched_characters) = response.json::<Vec<Character>>().await {
                        characters.set(fetched_characters);
                    }
                }
            })
        });
    }

    html! {
        <div>
            <Button {onclick}>{ "+1" }</Button>
            <p class={classes!("dark:text-white")}>{ *counter }</p>
            <ul class={classes!("dark:text-white")}>
                { characters.iter().filter_map(|c| c.name.as_ref() ).collect::<Html>() }
            </ul>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
