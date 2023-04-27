use derive_builder::Builder;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct Notification {
    message: AttrValue,
}

impl Notification {
    pub fn builder() -> NotificationBuilder {
        Default::default()
    }

    pub fn to_html(&self) -> Html {
        html_nested! {
            <span> {self.message.clone()} </span>
        }
    }
}
