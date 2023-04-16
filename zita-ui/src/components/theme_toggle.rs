use yew::prelude::*;

use super::Toggle;
use crate::theme::{use_theme, Theme};

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or(16)]
    pub size: u16,

    pub length: Option<u16>,
}

#[function_component]
pub fn ThemeToggle(props: &Props) -> Html {
    let theme = use_theme();

    use_effect({
        let theme = theme.clone();

        move || Theme::store(*theme)
    });

    html! {
        <Toggle<Theme> value={theme} states={(Theme::Light, Theme::Dark)} size={props.size} length={props.length} />
    }
}
