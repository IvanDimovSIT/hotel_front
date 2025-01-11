use std::sync::{Arc, Mutex};

use iced::{
    widget::{column, text},
    Element,
    Length::Fill,
    Task,
};

use crate::{
    app::{AppMessage, GlobalState, Screen},
    styles::TITLE_FONT_SIZE,
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
        message: AppMessage,
        global_state: Arc<Mutex<GlobalState>>,
    ) -> Task<AppMessage> {
        Task::none()
    }

    fn view(&self, global_state: Arc<Mutex<GlobalState>>) -> Element<AppMessage> {
        text!("Hello").size(TITLE_FONT_SIZE).width(Fill).into()
    }
}
