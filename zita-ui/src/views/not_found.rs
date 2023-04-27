use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yew_router::prelude::*;

use super::Route;
use crate::components::*;

#[function_component]
pub fn NotFound() -> Html {
    // Ensure we replace the URL by the exact URL of the not found page
    let navigator = use_navigator().expect("Unable to interact with Browser history, aborting.");
    navigator.replace(&Route::NotFound);

    html! {
        <Island>
            <Icon icon_id={IconId::LucideGhost} width="96px" height="96px" />

            <p>{"It seems the resource you're after does not exist."}</p>

            <p>
                <a href="javascript:history.back()" >
                    <Icon icon_id={IconId::LucideUndo} width="16px" height="16px" />
                    {" Save me from the void"}
                </a>
            </p>
        </Island>
    }
}
