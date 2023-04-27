use std::rc::Rc;

use yew::prelude::*;

use super::Notification;

#[derive(Debug, Default, PartialEq)]
pub struct NotificationState {
    notifications: Vec<Notification>,
}

impl NotificationState {
    pub fn to_html(&self) -> Html {
        self.notifications
            .iter()
            .map(Notification::to_html)
            .collect::<Html>()
    }
}

impl Reducible for NotificationState {
    type Action = Notification;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut notifications = self.notifications.clone();

        notifications.push(action);

        Self { notifications }.into()
    }
}
