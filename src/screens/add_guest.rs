use std::sync::{Arc, Mutex};

use iced::{
    widget::{button, column, row, text, text_input},
    Alignment::Center,
    Element,
    Length::Fill,
    Task,
};

use crate::{
    app::{AppMessage, GlobalState, Screen},
    components::text_box::text_box::TextBox,
    styles::{ERROR_COLOR, FORM_SPACING, TEXT_BOX_WIDTH, TITLE_FONT_SIZE},
};

#[derive(Debug, Clone)]
pub enum AddGuestMessage {
    ChangeFirstName(String),
    ChangeLastName(String),
}

pub struct AddGuestScreen {
    error: String,
    first_name_input: TextBox,
    last_name_input: TextBox,
}
impl AddGuestScreen {
    pub fn new() -> Self {
        Self {
            error: "".to_owned(),
            first_name_input: TextBox::new("", 20),
            last_name_input: TextBox::new("", 20),
        }
    }
}
impl Screen for AddGuestScreen {
    fn update(
        &mut self,
        message: AppMessage,
        global_state: Arc<Mutex<GlobalState>>,
    ) -> Task<AppMessage> {
        match message {
            AppMessage::AddGuestMessage(m) => match m {
                AddGuestMessage::ChangeFirstName(x) => {
                    self.first_name_input.update(x);
                    Task::none()
                }
                AddGuestMessage::ChangeLastName(x) => {
                    self.last_name_input.update(x);
                    Task::none()
                }
            },
            _ => Task::none(),
        }
    }

    fn view(&self, _global_state: Arc<Mutex<GlobalState>>) -> Element<AppMessage> {
        column![
            text!("Add Guest")
                .align_x(Center)
                .size(TITLE_FONT_SIZE)
                .width(Fill),
            text_input("First Name", self.first_name_input.get_text())
                .on_input(|x| AppMessage::AddGuestMessage(AddGuestMessage::ChangeFirstName(x)))
                .align_x(Center)
                .width(TEXT_BOX_WIDTH)
                .line_height(1.5),
            text_input("Last Name", self.last_name_input.get_text())
                .on_input(|x| AppMessage::AddGuestMessage(AddGuestMessage::ChangeLastName(x)))
                .align_x(Center)
                .width(TEXT_BOX_WIDTH)
                .line_height(1.5),
            text!("{}", self.error)
                .color(ERROR_COLOR)
                .size(18)
                .align_x(Center)
                .width(Fill),
            button("Add").height(30).width(80)
        ]
        .spacing(FORM_SPACING)
        .align_x(Center)
        .into()
    }
}
