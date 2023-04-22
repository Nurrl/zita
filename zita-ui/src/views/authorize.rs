use stylist::yew::*;
use web_sys::{FormData, HtmlFormElement};
use yew::prelude::*;
use yew_icons::{Icon, IconId};

use crate::components::*;

#[styled_component]
pub fn Authorize() -> Html {
    let onsubmit = Callback::from(|e: SubmitEvent| {
        let form = e.target_unchecked_into::<HtmlFormElement>();
        let data = FormData::new_with_form(&form).expect("Unable to select form data, aborting.");

        gloo::console::log!("Form trigerer:", e.submitter());
        gloo::console::log!("Form datas:", data);
        gloo::dialogs::alert("Submitted !");

        e.prevent_default();
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
                    <Button type_="submit" size="100%">
                        {"Sign in"}<Icon icon_id={ IconId::LucideChevronLast } />
                    </Button>
                </p>
            </form>

            <br />

            <p><ThemeToggle /></p>
        </Island>
    }
}
