use yew::prelude::*;
use yew_router::prelude::*;

mod authorize;
mod not_found;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/authorize")]
    Authorize,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Authorize => html! { <authorize::Authorize /> },
        Route::NotFound => html! { <not_found::NotFound /> },
    }
}
