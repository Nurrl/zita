use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod theme;
mod views;

use components::*;

#[function_component]
pub fn Root() -> Html {
    html! {
        <BrowserRouter>
            <ThemeProvider>
                <NotificationProvider>
                    <Switch<views::Route> render={views::switch} />
                </NotificationProvider>
            </ThemeProvider>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<Root>::new().render();
}
