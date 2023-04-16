use stylist::yew::*;
use yew::prelude::*;

use web_sys::HtmlInputElement;

use crate::theme::use_theme;

#[derive(Debug, PartialEq, Properties)]
pub struct Props<T>
where
    T: PartialEq,
{
    #[prop_or(16)]
    pub size: u16,

    pub length: Option<u16>,

    pub value: UseStateHandle<T>,
    pub states: (T, T),
}

#[function_component]
pub fn Toggle<T: Copy + PartialEq + 'static>(props: &Props<T>) -> Html {
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
        background: rgba(127, 127, 127, 0.5);

        cursor: pointer;
        width: calc(${length}px + 8px);
        height: calc(${size}px + 8px);

        display: block;
        position: relative;

        border: 2px solid ${fg};
        border-radius: ${size}px;
    }

    label:after {
        background: ${bg};
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
        background: ${fg};
    }

    input:checked + label:after {
        left: calc(100% - 4px);
        transform: translateX(-100%);
    }

    label:active:after {
        width: calc(${size}px + 10%);
    }
    "#,
        fg = theme.fg(),
        bg = theme.bg(),
        size = props.size,
        length = props.length.unwrap_or(props.size * 2)
    );

    let onchange = Callback::from({
        let value = props.value.clone();
        let states = props.states;

        move |event: Event| {
            let checked = event.target_unchecked_into::<HtmlInputElement>().checked();

            value.set(match checked {
                false => states.0,
                true => states.1,
            });
        }
    });

    let id = nanoid::nanoid!(10);

    html! {
        <span class={style}>
            <input type="checkbox" id={id.clone()} {onchange} checked={ *props.value == props.states.1 } />
            <label for={id}></label>
        </span>
    }
}
