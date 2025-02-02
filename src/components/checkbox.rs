use iced::{widget::checkbox, Element};

use crate::app::AppMessage;

pub struct Checkbox {
    name: String,
    is_checked: bool,
}
impl Checkbox {
    pub fn new<T>(name: T, is_checked: bool) -> Self
    where
        T: Into<String>,
    {
        Self {
            name: name.into(),
            is_checked,
        }
    }

    pub fn update(&mut self, is_checked: bool) {
        self.is_checked = is_checked;
    }

    pub fn view<'a, F>(&'a self, on_toggle: F) -> Element<AppMessage>
    where
        F: Fn(bool) -> AppMessage + 'a,
    {
        checkbox(&self.name, self.is_checked)
            .on_toggle(on_toggle)
            .into()
    }

    pub fn is_checked(&self) -> bool {
        self.is_checked
    }
}
