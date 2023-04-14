use stylist::yew::*;
use yew::prelude::*;

use super::Theme;

#[derive(Debug, PartialEq, Properties)]
pub struct ThemeProviderProps {
    pub children: Children,
}

#[styled_component]
pub fn ThemeProvider(props: &ThemeProviderProps) -> Html {
    let theme = use_state(Theme::default);

    let style = css!(
        r#"
    body {
        background: ${background};
        color: ${foreground};

        font-family: "Open Sans", sans-serif;
    }
    "#,
        foreground = theme.foreground,
        background = theme.background
    );

    html! {
        <>
            <Global css={style} />
            <ContextProvider<UseStateHandle<Theme>> context={theme}>
                {props.children.clone()}
            </ContextProvider<UseStateHandle<Theme>>>
        </>
    }
}
