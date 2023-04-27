use stylist::yew::*;
use yew::prelude::*;

use super::use_notification;

#[function_component]
pub fn NotificationContainer() -> Html {
    let state = use_notification();
    let notifications = state.to_html();

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

    html! {
        <div class={style}>
            {notifications}
        </div>
    }
}
