use stylist::yew::*;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod theme;
mod views;

use components::*;

#[styled_component]
pub fn Root() -> Html {
    html! {
        <BrowserRouter>
            <ThemeProvider>
                <Switch<views::Route> render={views::switch} />
            </ThemeProvider>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<Root>::new().render();
}
