use stylist::yew::*;
use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or(AttrValue::from("360px"))]
    pub width: AttrValue,

    pub children: Children,
}

#[function_component]
pub fn Island(props: &Props) -> Html {
    let style = use_style!(
        r#"
    display: flex;

    justify-content: center;
    align-items: center;

    width: 100%;
    height: 100%;

    > span {
        display: block;
        text-align: center;

        width: ${width};
    }
    "#,
        width = props.width
    );

    html! {
        <div class={style}>
            <span>
                {for props.children.iter()}
            </span>
        </div>
    }
}
