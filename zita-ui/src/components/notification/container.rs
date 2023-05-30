use stylist::yew::*;
use yew::prelude::*;

use super::use_notification;

#[function_component]
pub fn NotificationContainer() -> Html {
    let state = use_notification();

    let style = use_style!(
        r#"
    position: fixed;
    overflow: hidden;

    padding: 16px;

    bottom: 0;
    left: 50%;
    transform: translate(-50%, 0);

    pointer-events: none;

    span {
        pointer-events: auto;
        display: block;

        margin: 16px;
    }
    "#
    );

    // On state modification, ignite all notification that haven't been ignited yet.
    use_effect_with_deps(
        |state| {
            state
                .notifications()
                .for_each(|notification| notification.ignite(state.clone()))
        },
        state.clone(),
    );

    html! {
        <div class={style}>
            {state.to_html()}
        </div>
    }
}
