use std::{collections::HashMap, rc::Rc};

use yew::prelude::*;

use super::{Notification, NotificationId};

#[derive(Debug)]
pub enum NotificationAction {
    Create(Notification),
    Erase(NotificationId),
}

#[derive(Debug, Default, PartialEq)]
pub struct NotificationState {
    notifications: HashMap<NotificationId, Notification>,
}

impl NotificationState {
    pub fn notifications(&self) -> impl Iterator<Item = &Notification> {
        self.notifications.values()
    }

    pub fn to_html(&self) -> Html {
        self.notifications()
            .map(Notification::to_html)
            .collect::<Html>()
    }
}

impl Reducible for NotificationState {
    type Action = NotificationAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut notifications = self.notifications.clone();

        match action {
            NotificationAction::Create(message) => {
                notifications.insert(message.id(), message);
            }
            NotificationAction::Erase(id) => {
                notifications.remove(&id);
            }
        };

        Self { notifications }.into()
    }
}
