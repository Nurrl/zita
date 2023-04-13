use yew::prelude::*;

mod provider;
pub use provider::ThemeProvider;

#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    pub foreground: AttrValue,
    pub background: AttrValue,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            foreground: "#070A52".into(),
            background: "#F0F0F0".into(),
        }
    }
}

#[hook]
pub fn use_theme() -> UseStateHandle<Theme> {
    use_context::<UseStateHandle<Theme>>().unwrap()
}
