use stylist::yew::*;
use yew::prelude::*;

use crate::theme::use_theme;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or(36)]
    pub size: u16,

    #[prop_or(AttrValue::from("..."))]
    pub placeholder: AttrValue,

    #[prop_or_default]
    pub autofocus: bool,
}

#[function_component]
pub fn Input(props: &Props) -> Html {
    let theme = use_theme();

    let style = use_style!(
        r#"
    display: inline-flex;
    align-items: center;
    justify-content: center;

    background: ${background};

    padding: 12px;

    border-radius: 4px;
    box-shadow: 0 0 50px rgba(0, 0, 0, 0.25);

    input {
        background: none;
        color: ${foreground};

        font-size: 16px;

        outline: none;
        border: 0;
    }
    "#,
        foreground = theme.foreground,
        background = theme.background
    );

    html! {
        <span class={style}>
            <input type="text" size={props.size.to_string()} placeholder={props.placeholder.clone()} autofocus={props.autofocus} />
        </span>
    }
}
