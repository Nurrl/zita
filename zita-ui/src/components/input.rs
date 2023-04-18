use stylist::yew::*;
use yew::prelude::*;

use crate::theme::use_theme;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or(AttrValue::from("320px"))]
    pub size: AttrValue,

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

    background: ${bg};

    width: ${size};
    padding: 12px;

    border-radius: 4px;
    box-shadow: 0 0 50px rgba(0, 0, 0, 0.25);

    input {
        width: 100%;
        font-size: 16px;

        background: none;
        color: ${fg};

        outline: none;
        border: 0;
    }
    "#,
        fg = theme.fg(),
        bg = theme.bg(),
        size = props.size
    );

    html! {
        <span class={style}>
            <input
                type="text"
                placeholder={props.placeholder.clone()}
                autofocus={props.autofocus} />
        </span>
    }
}
