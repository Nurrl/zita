use yew::prelude::*;

use crate::components::*;

#[function_component]
pub fn ComponentMuseum() -> Html {
    let value = use_state(|| false);

    html! {
        <Island>
            <p><ThemeToggle /></p>

            <p>
                <Button>{"Very nice button"}</Button>
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
