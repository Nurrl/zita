use yew::prelude::*;

use crate::components::*;

#[function_component]
pub fn ComponentMuseum() -> Html {
    let notification = use_notification();
    let value = use_state_eq(|| false);

    html! {
        <Island>
            <p><ThemeToggle /></p>

            <p>
                <Button onclick={
                    Callback::from(move |_| notification.dispatch(Notification::builder().message("Hello from the notifications").build().unwrap()))
                }>{"Very nice button"}</Button>
            </p>

            <p><Input name="very-nice" /></p>

            <p>{"Spinner: "}<Spinner /></p>

            <p>
                <Toggle<bool> states={(false, true)} value={value.clone()} />

                <br />

                {
                    if *value {
                        "on"
                    } else {
                        "off"
                    }
                }
            </p>
        </Island>
    }
}
