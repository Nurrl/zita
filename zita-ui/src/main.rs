use stylist::yew::*;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

mod components;
use components::*;

#[styled_component]
pub fn App() -> Html {
    let css = css!(
        "
    --background: #F0F0F0;
    --foreground: #070A52;

    --primary: #6F1AB6;

    body {
        background: var(--background);
    }
    "
    );

    html! {
        <>
            <Global css={css} />
            <div>
                <Input placeholder="Email address" autofocus=true />
                <br /><br />
                <Input placeholder="Password" />
                <Button>{"Sign in"}<Icon icon_id={IconId::LucideChevronLast} width="16px" height="16px" /></Button>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
