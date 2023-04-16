use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use yew::prelude::*;

mod provider;
pub use provider::ThemeProvider;

const THEME_STORAGE_KEY: &'static str = "theme";

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

impl Theme {
    pub fn load() -> Theme {
        LocalStorage::get(THEME_STORAGE_KEY).unwrap_or_default()
    }

    pub fn store(theme: Theme) {
        LocalStorage::set(THEME_STORAGE_KEY, theme)
            .expect("Unable to store theming preferences to local storage")
    }

    pub fn fg(&self) -> &'static str {
        match self {
            Self::Light => "#070A52",
            Self::Dark => "#F0F0F0",
        }
    }

    pub fn bg(&self) -> &'static str {
        match self {
            Self::Light => "#F0F0F0",
            Self::Dark => "#070A52",
        }
    }
}

#[hook]
pub fn use_theme() -> UseStateHandle<Theme> {
    use_context::<UseStateHandle<Theme>>().unwrap()
}
