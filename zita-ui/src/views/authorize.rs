use stylist::yew::*;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

use crate::components::*;

#[styled_component]
pub fn Authorize() -> Html {
    html! {
        <Island>
            <p><h1>{"Sign in to your account"}</h1></p>

            <br />

            <p><Input size="100%" placeholder="Email address" autofocus=true /></p>
            <p><Input size="100%" placeholder="Password" /></p>

            <br />

            <p><Button size="100%">{"Sign in"}<Icon icon_id={ IconId::LucideChevronLast } /></Button></p>

            <br />

            <p><ThemeToggle /></p>
        </Island>
    }
}
