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
    #[prop_or(AttrValue::from("24px"))]
    pub size: AttrValue,
    pub length: Option<AttrValue>,

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
        display: none;
    }

    label {
        background: rgba(127, 127, 127, 0.5);

        cursor: pointer;
        width: calc(${length} + 8px);
        height: calc(${size} + 8px);

        display: block;
        position: relative;

        border: 2px solid ${text};
        border-radius: ${size};
    }

    label > span {
        background: ${background};

        top: 4px;
        left: 4px;
        width: ${size};
        height: ${size};

        border-radius: ${size};

        position: absolute;
        display: inline-flex;
        justify-content: center;
        align-items: center;

        transition: 0.2s;
    }

    input:checked + label {
        background: ${text};
    }

    input:checked + label > span {
        left: calc(100% - 4px);
        transform: translateX(-100%);
    }

    label:active > span {
        width: calc(${size} + 5%);
    }
    "#,
        text = theme.text(),
        background = theme.background(),
        size = props.size,
        length = props
            .length
            .as_ref()
            .unwrap_or(&format!("calc({} * 2)", props.size).into())
    );

    let id = use_memo(|_| nanoid::nanoid!(10), ());
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
                    width={format!("calc({} - 2px)", props.size)}
                    height={format!("calc({} - 2px)", props.size)} />
            }
        });

    html! {
        <span class={style}>
            <input
                type="checkbox"
                id={(*id).clone()}
                {onchange}
                {checked} />

            <label for={(*id).clone()}><span>{icon}</span></label>
        </span>
    }
}
