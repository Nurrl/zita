use stylist::yew::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use super::use_theme;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or(AttrValue::from("text"))]
    pub type_: AttrValue,

    pub name: AttrValue,

    #[prop_or(AttrValue::from("..."))]
    pub placeholder: AttrValue,

    #[prop_or(AttrValue::from("320px"))]
    pub size: AttrValue,

    #[prop_or_default]
    pub required: bool,

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

    background: ${contrast};

    width: ${size};

    border-radius: 4px;
    box-shadow: 0 0 50px rgba(0, 0, 0, 0.25);

    input {
        width: 100%;
        font-size: 16px;

        background: none;
        color: ${text};

        padding: 12px;
        outline: none;
        border: 0;
    }
    "#,
        text = theme.text(),
        contrast = theme.contrast(),
        size = props.size
    );

    let input = use_node_ref();

    // Use of `use_effect_with_deps` and `()` as deps to only
    // run on the first render and not later re-draws.
    use_effect_with_deps(
        {
            let input = input.clone();
            let autofocus = props.autofocus;

            move |_| {
                if autofocus {
                    input
                        .cast::<HtmlInputElement>()
                        .unwrap()
                        .focus()
                        .expect("Unable to focus the input field, aborting.");
                }
            }
        },
        (),
    );

    let required = props.required;

    html! {
        <span class={style}>
            <input
                ref={input}
                name={props.name.clone()}
                type={props.type_.clone()}
                placeholder={props.placeholder.clone()}
                {required} />
        </span>
    }
}
