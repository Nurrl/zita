use stylist::yew::*;
use yew::prelude::*;

use super::{state::NotificationAction, use_notification};

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

    use_effect_with_deps(
        |state| {
            for (id, duration) in state
                .notifications()
                .map(|notification| {
                    notification
                        .take_duration()
                        .map(|duration| (notification.id(), duration))
                })
                .flatten()
            {
                yew::platform::spawn_local({
                    let state = state.clone();

                    async move {
                        yew::platform::time::sleep(duration).await;

                        state.dispatch(NotificationAction::Erase(id))
                    }
                });
            }
        },
        state.clone(),
    );

    html! {
        <div class={style}>
            {state.to_html()}
        </div>
    }
}
