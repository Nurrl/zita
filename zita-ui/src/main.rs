use stylist::yew::*;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

mod components;
use components::*;

mod theme;
use theme::{use_theme, ThemeProvider};

#[styled_component]
pub fn App() -> Html {
    let theme = use_theme();

    let style = css!(
        r#"
        body {
            background: ${background};
            color: ${foreground};
        }
    "#,
        foreground = theme.foreground,
        background = theme.background
    );

    html! {
        <>
            <Global css={style} />
            <div>
                <Toggle />
                <br /><br />
                <Input placeholder="Email address" autofocus=true />
                <br /><br />
                <Input placeholder="Password" />
                <Button>{"Sign in"}<Icon icon_id={IconId::LucideChevronLast} width="16px" height="16px" /></Button>
            </div>
        </>
    }
}

#[styled_component]
pub fn Root() -> Html {
    html! {
        <ThemeProvider>
            <App />
        </ThemeProvider>
    }
}

fn main() {
    yew::Renderer::<Root>::new().render();
}
