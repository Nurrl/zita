use std::cell::Cell;
use std::time::Duration;

use derive_builder::Builder;
use yew::prelude::*;

mod provider;
pub use provider::{use_notification, NotificationProvider};

mod state;
pub use state::NotificationState;

mod container;
pub use container::NotificationContainer;

use state::NotificationAction;

type NotificationId = u16;

#[derive(Debug, Clone, PartialEq, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct Notification {
    #[builder(setter(skip), default = "fastrand::u16(..)")]
    id: NotificationId,
    #[builder(default = "Some(Duration::from_secs(3)).into()")]
    duration: Cell<Option<Duration>>,

    message: AttrValue,
}

impl Notification {
    pub fn builder() -> NotificationBuilder {
        Default::default()
    }

    pub fn id(&self) -> NotificationId {
        self.id
    }

    pub fn take_duration(&self) -> Option<Duration> {
        self.duration.replace(None)
    }

    pub fn to_html(&self) -> Html {
        html_nested! {
            <span>
                {self.message.clone()}
            </span>
        }
    }
}

impl Into<NotificationAction> for Notification {
    fn into(self) -> NotificationAction {
        NotificationAction::Create(self)
    }
}
