use stylist::yew::*;
use yew::prelude::*;

use super::use_theme;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or(AttrValue::from("24px"))]
    pub size: AttrValue,
}

#[function_component]
pub fn Spinner(props: &Props) -> Html {
    let theme = use_theme();

    let style = use_style!(
        r#"
    display: inline-flex;
    width: ${size};
    height: ${size};

    justify-content: center;
    align-items: center;

    span {
        display: block;

        width: calc(${size} / 3);
        height: calc(${size} / 3);
        border-radius: 50%;
        box-shadow: calc(${size} / 3) 0 0 0 ${text}, calc(-${size} / 3) 0 0 0 ${text};
        animation: spinner 1s infinite;
    }

    @keyframes spinner {
        to {
            transform: rotate(360deg);
        }
    }
    "#,
        size = props.size,
        text = theme.text()
    );

    html! {
        <div class={style}>
            <span></span>
        </div>
    }
}
