mod games;
mod home;
mod not_found;

use crate::app::{games::Games, home::Home, not_found::NotFound};
use crate::components::header::Header;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
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
        AppRoute::Home => html! { <Home /> },
        AppRoute::Games => html! { <Games /> },
        AppRoute::NotFound => html! { <NotFound /> },
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <div class="flex flex-col min-h-screen">
            <BrowserRouter>
                <Header />
                <Switch<AppRoute> render={switch} />
            </BrowserRouter>
        </div>
    }
}
