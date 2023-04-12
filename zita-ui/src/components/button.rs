use stylist::yew::*;
use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or(36)]
    pub size: u16,

    #[prop_or_default]
    pub on_click: Callback<Event>,

    pub children: Children,
}

#[function_component]
pub fn Button(props: &Props) -> Html {
    let style = use_style!(
        "
    display: inline-flex;
    align-items: center;
    justify-content: center;

    background: var(--foreground);
    color: var(--background);

    font-size: 16px;
    padding: 11px;

    border: 2px solid var(--foreground);
    border-radius: 8px;
    box-shadow: 0 0 50px rgba(0, 0, 0, 0.25);

    transition: 0.15s;

    :hover {
        background: transparent;
        color: var(--foreground);

        cursor: pointer;
    }
        "
    );

    html! {
        <button class={style}>{for props.children.iter()}</button>
    }
}
