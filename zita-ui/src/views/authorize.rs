use stylist::yew::*;
use web_sys::{FormData, HtmlFormElement};
use yew::prelude::*;
use yew_icons::{Icon, IconId};

use crate::components::*;

#[styled_component]
pub fn Authorize() -> Html {
    let loading = use_state(|| false);

    let onsubmit = Callback::from({
        let loading = loading.clone();

        move |e: SubmitEvent| {
            loading.set(true);

            let form = e.target_unchecked_into::<HtmlFormElement>();
            let data =
                FormData::new_with_form(&form).expect("Unable to select form data, aborting.");

            gloo::console::debug!(e.submitter());
            gloo::console::debug!(data);

            yew::platform::spawn_local({
                let loading = loading.clone();

                async move {
                    yew::platform::time::sleep(std::time::Duration::from_secs(3)).await;

                    loading.set(false);
                }
            });

            e.prevent_default();
        }
    });

    html! {
        <Island>
            <p><h1>{"Sign in to your account"}</h1></p>

            <br />

            <form {onsubmit}>
                <p>
                    <Input
                        type_="email"
                        name="email"
                        placeholder="Email address"
                        size="100%"
                        required=true
                        autofocus=true />
                </p>
                <p>
                    <Input
                        type_="password"
                        name="password"
                        placeholder="Password"
                        size="100%"
                        required=true />
                </p>

                <br />

                <p>
                    <Button type_="submit" disabled={*loading} size="100%"> {
                        if *loading {
                            html! { <Spinner /> }
                        } else {
                            html!{
                                <>
                                    {"Sign in"}
                                    <Icon icon_id={ IconId::LucideChevronLast } />
                                </>
                            }
                        }
                    } </Button>
                </p>
            </form>

            <br />

            <p><ThemeToggle /></p>
        </Island>
    }
}
