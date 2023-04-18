use stylist::yew::*;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

use crate::components::*;

#[styled_component]
pub fn Authorize() -> Html {
    let remember = use_state(|| true);

    html! {
        <Island>
            <h1>{"Sign in to your account"}</h1>
            <br /><br />
            <Input placeholder="Email address" autofocus=true />
            <br /><br />
            <Input placeholder="Password" />
            <br /><br />
            <Toggle<bool> value={remember} states={ (false, true) } />
            <br /><br />
            <Button size="100%">{"Sign in"}<Icon icon_id={IconId::LucideChevronLast} width="16px" height="16px" /></Button>
        </Island>
    }
}
