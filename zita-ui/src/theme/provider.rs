use stylist::yew::*;
use yew::prelude::*;

use super::Theme;

#[derive(Debug, PartialEq, Properties)]
pub struct ThemeProviderProps {
    pub children: Children,
}

#[styled_component]
pub fn ThemeProvider(props: &ThemeProviderProps) -> Html {
    let theme = use_state_eq(Theme::load);

    let style = css!(
        r#"
    html, body {
        height: 100%;

        background: ${bg};
        color: ${fg};

        font-family: "Open Sans", sans-serif;
    }

    body {
        margin: 0;
    }
    "#,
        fg = theme.fg(),
        bg = theme.bg()
    );

    html! {
        <ContextProvider<UseStateHandle<Theme>> context={theme}>
            <Global css={style} />
            {for props.children.iter()}
        </ContextProvider<UseStateHandle<Theme>>>
    }
}
