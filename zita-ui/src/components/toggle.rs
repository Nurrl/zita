use stylist::yew::*;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

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

    pub icons: Option<(IconId, IconId)>,
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

    label span {
        background: ${bg};

        top: 4px;
        left: 4px;
        width: ${size}px;
        height: ${size}px;

        border-radius: ${size}px;

        position: absolute;
        display: inline-flex;
        justify-content: center;
        align-items: center;

        transition: 0.2s;
    }

    input:checked + label {
        background: ${fg};
    }

    input:checked + label span {
        left: calc(100% - 4px);
        transform: translateX(-100%);
    }

    label:active span {
        width: calc(${size}px + 10%);
    }
    "#,
        fg = theme.fg(),
        bg = theme.bg(),
        size = props.size,
        length = props.length.unwrap_or(props.size * 2)
    );

    let id = nanoid::nanoid!(10);
    let checked = *props.value == props.states.1;
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

    let icon = props
        .icons
        .map(|(off, on)| match checked {
            false => off,
            true => on,
        })
        .map(|icon_id| {
            html! {
                <Icon
                    {icon_id}
                    width={format!("{}px", props.size - 2)}
                    height={format!("{}px", props.size - 2)} />
            }
        });

    html! {
        <span class={style}>
            <input
                type="checkbox"
                id={id.clone()}
                {onchange}
                {checked} />

            <label for={id}><span>{icon}</span></label>
        </span>
    }
}
