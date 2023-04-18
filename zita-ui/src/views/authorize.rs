use stylist::yew::*;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

use crate::components::*;

#[styled_component]
pub fn Authorize() -> Html {
    html! {
        <Island>
            <ThemeToggle size="24px" />
            <br /><br />
            <Input placeholder="Email address" autofocus=true />
            <br /><br />
            <Input placeholder="Password" />
            <br /><br />
            <Button size="100%">{"Sign in"}<Icon icon_id={IconId::LucideChevronLast} width="16px" height="16px" /></Button>
        </Island>
    }
}
