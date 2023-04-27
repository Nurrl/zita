use yew::prelude::*;
use yew_router::prelude::*;

mod authorize;
#[cfg(debug_assertions)]
mod components_museum;
mod not_found;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/authorize")]
    Authorize,

    #[cfg(debug_assertions)]
    #[at("/component-museum")]
    ComponentMuseum,

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Authorize => html! { <authorize::Authorize /> },
        #[cfg(debug_assertions)]
        Route::ComponentMuseum => html! { <components_museum::ComponentMuseum /> },
        Route::NotFound => html! { <not_found::NotFound /> },
    }
}
