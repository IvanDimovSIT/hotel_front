use iced::{widget::{column, text}, Task};

use crate::app::Screen;

pub struct HomeScreen;
impl HomeScreen {
    pub fn new() -> Self {
        Self
    }
}
impl Screen for HomeScreen{
    fn update(&mut self, message: crate::app::AppMessage, global_state: &mut crate::app::GlobalState) -> iced::Task<crate::app::AppMessage> {
        Task::none()
    }

    fn view(&self, global_state: &crate::app::GlobalState) -> iced::Element<crate::app::AppMessage> {
        text!("Hello")
            .into()
    }
}
    