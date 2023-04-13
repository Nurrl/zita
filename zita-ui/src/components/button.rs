use stylist::yew::*;
use yew::prelude::*;

use crate::theme::use_theme;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or(36)]
    pub size: u16,

    pub children: Children,
}

#[function_component]
pub fn Button(props: &Props) -> Html {
    let theme = use_theme();

    let style = use_style!(
        r#"
    display: inline-flex;
    align-items: center;
    justify-content: center;

    background: ${foreground};
    color: ${background};

    font-size: 16px;
    padding: 11px;

    border: 2px solid ${foreground};
    border-radius: 8px;
    box-shadow: 0 0 50px rgba(0, 0, 0, 0.25);

    transition: 0.15s;

    :hover {
        background: transparent;
        color: ${foreground};

        cursor: pointer;
    }
    "#,
        foreground = theme.foreground,
        background = theme.background
    );

    html! {
        <button class={style}>{for props.children.iter()}</button>
    }
}
