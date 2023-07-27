mod games;
mod home;
mod not_found;

use crate::app::{games::game::Game, games::Games, home::Home, not_found::NotFound};
use crate::components::header::Header;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/games")]
    GamesRoot,
    #[at("/games/*")]
    Games,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq)]
pub enum GamesRoute {
    #[at("/games")]
    Main,
    #[at("/games/*name")]
    Game { name: String },
    #[not_found]
    #[at("/games/404")]
    NotFound,
}

fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! { <Home /> },
        AppRoute::GamesRoot | AppRoute::Games => {
            html! { <Switch<GamesRoute> render={switch_games} /> }
        }
        AppRoute::NotFound => html! { <NotFound /> },
    }
}

fn switch_games(routes: GamesRoute) -> Html {
    match routes {
        GamesRoute::Main => html! { <Games /> },
        GamesRoute::Game { name } => html! { <Game /> },
        GamesRoute::NotFound => html! { <Redirect<AppRoute> to={AppRoute::NotFound} /> },
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Header />
            <Switch<AppRoute> render={switch} />
        </BrowserRouter>
    }
}
