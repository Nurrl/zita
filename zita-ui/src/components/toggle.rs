use stylist::yew::*;
use yew::prelude::*;

use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

use crate::theme::use_theme;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or(16)]
    pub size: u16,

    pub length: Option<u16>,

    pub callback: Option<Callback<bool>>,
}

#[function_component]
pub fn Toggle(props: &Props) -> Html {
    let theme = use_theme();

    let style = use_style!(
        r#"
    display: inline-block;
    vertical-align: middle;

    input[type=checkbox] {
        height: 0;
        width: 0;

        display: none;
    }

    label {
        background: lightgrey;

        cursor: pointer;
        width: calc(${length}px + 8px);
        height: calc(${size}px + 8px);

        display: block;
        position: relative;

        border: 2px solid ${foreground};
        border-radius: ${size}px;
    }

    label:after {
        background: ${background};
        content: "";

        top: 4px;
        left: 4px;
        width: ${size}px;
        height: ${size}px;

        position: absolute;
        border-radius: ${size}px;

        transition: 0.2s;
    }

    input:checked + label {
        background: ${foreground};
    }

    input:checked + label:after {
        left: calc(100% - 4px);
        transform: translateX(-100%);
    }

    label:active:after {
        width: calc(${size}px + 10%);
    }
    "#,
        foreground = theme.foreground,
        background = theme.background,
        size = props.size,
        length = props.length.unwrap_or(props.size * 2)
    );

    let onchange = props.callback.clone().map(|callback| {
        Callback::from(move |event: Event| {
            if let Some(Ok(target)) = event
                .target()
                .map(|target| target.dyn_into::<HtmlInputElement>())
            {
                callback.emit(target.checked());
            }
        })
    });

    let id = nanoid::nanoid!(10);

    html! {
        <span class={style}>
            <input type="checkbox" id={id.clone()} {onchange} />
            <label for={id}></label>
        </span>
    }
}
