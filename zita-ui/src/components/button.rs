use stylist::yew::*;
use yew::prelude::*;

use super::use_theme;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or(AttrValue::from("button"))]
    pub type_: AttrValue,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or(AttrValue::from("96px"))]
    pub size: AttrValue,

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

    background: ${text};
    color: ${background};

    width: ${size};
    font-size: 16px;
    padding: 11px;

    border: 2px solid ${text};
    border-radius: 8px;
    box-shadow: 0 0 50px rgba(0, 0, 0, 0.25);

    transition: 0.15s;

    :hover:enabled {
        background: ${contrast};
        color: ${text};

        cursor: pointer;
    }

    :disabled {
        background: ${contrast};
        border-color: ${contrast};
        opacity: 0.6;

        transition: 0.2s;
    }
    "#,
        text = theme.text(),
        background = theme.background(),
        contrast = theme.contrast(),
        size = props.size
    );

    let disabled = props.disabled;

    html! {
        <button
            type={props.type_.clone()}
            class={style}
            {disabled}>
            {for props.children.iter()}
        </button>
    }
}
