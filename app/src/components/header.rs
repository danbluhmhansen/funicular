use crate::app::AppRoute;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn Header() -> Html {
    html! {
        <header>
            <div class={classes!("navbar")}>
                <ul class={classes!("navbar-start")}></ul>
                <ul class={classes!("navbar-center", "menu", "menu-horizontal")}>
                    <li><Link<AppRoute> to={AppRoute::Home}>{ "Home" }</Link<AppRoute>></li>
                    <li><Link<AppRoute> to={AppRoute::Games}>{ "Games" }</Link<AppRoute>></li>
                </ul>
                <ul class={classes!("navbar-end")}></ul>
            </div>
        </header>
    }
}
