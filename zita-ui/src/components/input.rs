use stylist::yew::*;
use yew::prelude::*;

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
    let autofocus = props.autofocus;

    let style = use_style!(
        "
    display: inline-flex;
    align-items: center;
    justify-content: center;

    background: var(--background);

    padding: 12px;

    border-radius: 4px;
    box-shadow: 0 0 50px rgba(0, 0, 0, 0.25);

    input {
        background: none;
        color: var(--foreground);

        font-size: 16px;

        outline: none;
        border: 0;
    }
        "
    );

    html! {
        <div class={style}>
            <input type="text" size={props.size.to_string()} placeholder={props.placeholder.clone()} {autofocus} />
        </div>
    }
}
