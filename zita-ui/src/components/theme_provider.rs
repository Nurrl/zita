use stylist::yew::*;
use yew::prelude::*;

use crate::theme::Theme;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[styled_component]
pub fn ThemeProvider(props: &Props) -> Html {
    let theme = use_state_eq(Theme::load);

    let style = css!(
        r#"
    body {
        background: ${background};
        color: ${text};
    }
    "#,
        background = theme.background(),
        text = theme.text()
    );

    html! {
        <ContextProvider<UseStateHandle<Theme>> context={theme}>
            <Global css={style} />
            {for props.children.iter()}
        </ContextProvider<UseStateHandle<Theme>>>
    }
}

#[hook]
pub fn use_theme() -> UseStateHandle<Theme> {
    use_context::<UseStateHandle<Theme>>().unwrap()
}
