use yew::prelude::*;

mod imp;
pub use imp::Notification;

mod provider;
pub use provider::NotificationProvider;

mod state;
pub use state::NotificationState;

mod container;
pub use container::NotificationContainer;

#[hook]
pub fn use_notification() -> UseReducerHandle<NotificationState> {
    use_context::<UseReducerHandle<NotificationState>>().unwrap()
}
