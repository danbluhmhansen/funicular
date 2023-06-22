use gloo_net::http::Request;
use serde::Deserialize;
use yew::{prelude::*, suspense::use_future};

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
fn Characters() -> HtmlResult {
    let res = use_future(|| async {
        Request::get("http://localhost:3000/character")
            .send()
            .await?
            .json::<Vec<Character>>()
            .await
    })?;
    Ok(match *res {
        Ok(ref res) => html! {
            <table class={classes!("table-auto", "dark:text-white")}>
                <thead>
                    <tr>
                        <th>{"Id"}</th>
                        <th>{"Name"}</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        res.iter().map(|c| html! {
                            <tr>
                                <td>{c.id.as_ref()}</td>
                                <td>{c.name.as_ref()}</td>
                            </tr>
                        }).collect::<Html>()
                    }
                </tbody>
            </table>
        },
        Err(ref failure) => failure.to_string().into(),
    })
}

#[function_component]
fn App() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};
    html! {
        <div>
            <Suspense {fallback}>
                <Characters />
            </Suspense>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
