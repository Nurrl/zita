use yew::prelude::*;
use yew_icons::IconId;

use super::{use_theme, Toggle};
use crate::theme::Theme;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or(AttrValue::from("24px"))]
    pub size: AttrValue,
    pub length: Option<AttrValue>,
}

#[function_component]
pub fn ThemeToggle(props: &Props) -> Html {
    let theme = use_theme();

    use_effect_with_deps(move |theme| Theme::store(**theme), theme.clone());

    html! {
        <Toggle<Theme>
            value={theme}
            states={ (Theme::Light, Theme::Dark) }
            icons={ (IconId::LucideSunDim, IconId::LucideMoon) }
            size={props.size.clone()} length={props.length.clone()} />
    }
}
