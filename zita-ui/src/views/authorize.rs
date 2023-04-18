use stylist::yew::*;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

use crate::components::*;

#[styled_component]
pub fn Authorize() -> Html {
    html! {
        <Island>
            <ThemeToggle size=20 />
            <br /><br />
            <Input placeholder="Email address" autofocus=true />
            <br /><br />
            <Input placeholder="Password" />
            <br /><br />
            <Button>{"Sign in"}<Icon icon_id={IconId::LucideChevronLast} width="16px" height="16px" /></Button>
        </Island>
    }
}
