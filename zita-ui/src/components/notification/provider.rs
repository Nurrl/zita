use yew::prelude::*;

use super::{NotificationContainer, NotificationState};

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[function_component]
pub fn NotificationProvider(props: &Props) -> Html {
    let notifications = use_reducer_eq(NotificationState::default);

    html! {
        <ContextProvider<UseReducerHandle<NotificationState>> context={notifications}>
            {for props.children.iter()}
            <NotificationContainer />
        </ContextProvider<UseReducerHandle<NotificationState>>>
    }
}

#[hook]
pub fn use_notification() -> UseReducerHandle<NotificationState> {
    use_context::<UseReducerHandle<NotificationState>>().unwrap()
}
