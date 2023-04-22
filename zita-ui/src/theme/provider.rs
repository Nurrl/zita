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

        background: ${background};
        color: ${text};

        font-family: "Open Sans", sans-serif;
    }

    body {
        margin: 0;
    }
    "#,
        text = theme.text(),
        background = theme.background()
    );

    html! {
        <ContextProvider<UseStateHandle<Theme>> context={theme}>
            <Global css={style} />
            {for props.children.iter()}
        </ContextProvider<UseStateHandle<Theme>>>
    }
}
