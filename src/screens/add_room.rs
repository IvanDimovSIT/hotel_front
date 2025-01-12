use std::sync::{Arc, Mutex};

use iced::{
    widget::{button, column, pick_list, text, text_input},
    Alignment::Center,
    Element,
    Length::Fill,
    Task,
};

use crate::{
    app::{AppMessage, GlobalState, Screen},
    components::{
        combo_box::bathroom_type_combo_box::{BathroomType, BathroomTypeComboBox},
        text_box::{
            number_text_box::{NumberTextBox, NumberType},
            room_number_text_box::RoomNumberTextBox,
        },
    },
    styles::{ERROR_COLOR, TEXT_BOX_WIDTH, TITLE_FONT_SIZE},
};

#[derive(Debug, Clone)]
pub enum AddRoomMessage {
    ChangeRoomNumber(String),
    ChangeFloor(String),
    ChagePrice(String),
    ChangeBathroomType(BathroomType),
}

pub struct AddRoomScreen {
    price: NumberTextBox,
    floor: NumberTextBox,
    room_number: RoomNumberTextBox,
    bathroom_type_combo_box: BathroomTypeComboBox,
    error: Arc<Mutex<String>>,
}
impl AddRoomScreen {
    pub fn new() -> Self {
        Self {
            price: NumberTextBox::new("", 9, NumberType::Price),
            floor: NumberTextBox::new("", 6, NumberType::PositiveInteger),
            room_number: RoomNumberTextBox::new(""),
            bathroom_type_combo_box: BathroomTypeComboBox::new(),
            error: Arc::new(Mutex::new("".to_owned())),
        }
    }
}
impl Screen for AddRoomScreen {
    fn update(
        &mut self,
        message: AppMessage,
        global_state: Arc<Mutex<GlobalState>>,
    ) -> Task<AppMessage> {
        match message {
            AppMessage::AddRoomMessage(add_room_message) => match add_room_message {
                AddRoomMessage::ChangeRoomNumber(room_number) => {
                    self.room_number.update(room_number);
                    Task::none()
                }
                AddRoomMessage::ChangeFloor(floor) => {
                    self.floor.update(floor);
                    Task::none()
                }
                AddRoomMessage::ChagePrice(price) => {
                    self.price.update(price);
                    Task::none()
                }
                AddRoomMessage::ChangeBathroomType(bathroom_type) => {
                    self.bathroom_type_combo_box.update(bathroom_type);
                    Task::none()
                }
            },
            _ => Task::none(),
        }
    }

    fn view(&self, _global_state: Arc<Mutex<GlobalState>>) -> Element<AppMessage> {
        column![
            text!("Add Room")
                .align_x(Center)
                .size(TITLE_FONT_SIZE)
                .width(Fill),
            text_input("Room Number", self.room_number.get_text())
                .on_input(|x| AppMessage::AddRoomMessage(AddRoomMessage::ChangeRoomNumber(x)))
                .align_x(Center)
                .width(TEXT_BOX_WIDTH)
                .line_height(1.5),
            text_input("Floor", self.floor.get_text())
                .on_input(|x| AppMessage::AddRoomMessage(AddRoomMessage::ChangeFloor(x)))
                .align_x(Center)
                .width(TEXT_BOX_WIDTH)
                .line_height(1.5),
            text_input("Price", self.price.get_text())
                .on_input(|x| AppMessage::AddRoomMessage(AddRoomMessage::ChagePrice(x)))
                .align_x(Center)
                .width(TEXT_BOX_WIDTH)
                .line_height(1.5),
            text!("Bathroom type:"),
            self.bathroom_type_combo_box
                .view(|x| AppMessage::AddRoomMessage(AddRoomMessage::ChangeBathroomType(x)))
                .width(TEXT_BOX_WIDTH),
            text!("{}", self.error.lock().unwrap())
                .color(ERROR_COLOR)
                .size(18)
                .align_x(Center)
                .width(Fill),
            button("Add").height(30).width(80)
        ]
        .spacing(20)
        .align_x(Center)
        .into()
    }
}
