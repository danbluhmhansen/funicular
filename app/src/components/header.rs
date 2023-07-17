use crate::app::AppRoute;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn Header() -> Html {
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
