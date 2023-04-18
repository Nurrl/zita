use yew::prelude::*;
use yew_icons::IconId;

use super::Toggle;
use crate::theme::{use_theme, Theme};

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or(AttrValue::from("24px"))]
    pub size: AttrValue,
    pub length: Option<AttrValue>,
}

#[function_component]
pub fn ThemeToggle(props: &Props) -> Html {
    let theme = use_theme();

    use_effect({
        let theme = theme.clone();

        move || Theme::store(*theme)
    });

    html! {
        <Toggle<Theme>
            value={theme}
            states={(Theme::Light, Theme::Dark)}
            icons={(IconId::LucideSunDim, IconId::LucideMoon)}
            size={props.size.clone()} length={props.length.clone()} />
    }
}
