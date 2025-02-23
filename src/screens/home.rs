use std::sync::{Arc, Mutex};

use iced::{
    widget::{column, text},
    Alignment::Center,
    Element,
    Length::Fill,
    Task,
};

use crate::{
    app::{AppMessage, GlobalState, Screen},
    styles::{FORM_PADDING, FORM_SPACING, TITLE_FONT_SIZE},
};

pub struct HomeScreen;
impl HomeScreen {
    pub fn new() -> Self {
        Self
    }
}
impl Screen for HomeScreen {
    fn update(
        &mut self,
        _message: AppMessage,
        _global_state: Arc<Mutex<GlobalState>>,
    ) -> Task<AppMessage> {
        Task::none()
    }

    fn view(&self, _global_state: Arc<Mutex<GlobalState>>) -> Element<AppMessage> {
        column![text!("Hotel App")
            .size(TITLE_FONT_SIZE)
            .width(Fill)
            .align_x(Center)]
        .spacing(FORM_SPACING)
        .padding(FORM_PADDING)
        .into()
    }
}
