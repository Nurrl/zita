use stylist::yew::*;
use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yew_router::prelude::*;

use super::Route;
// use crate::components::*;

#[styled_component]
pub fn NotFound() -> Html {
    // Ensure we replace the URL by the exact URL of the not found page
    let navigator = use_navigator().expect("Unable to interact with Browser history");
    navigator.replace(&Route::NotFound);

    html! {
        <div>
            <p>
                { "It seems the resource you're after does not exist " }
                <Icon icon_id={IconId::LucideTornado} width="16px" height="16px" />
            </p>

            <p>
                <Link<Route> to={Route::Home}>
                    <Icon icon_id={IconId::LucideUndo} width="16px" height="16px" />
                    { "Save me from the void" }
                </Link<Route>>
            </p>
        </div>
    }
}
