use iced::{
    keyboard::{key::Named, on_key_press},
    Subscription,
};

use crate::app::{AppMessage, HotelApp};

pub fn select_text_input_subscription(_hotel_app: &HotelApp) -> Subscription<AppMessage> {
    on_key_press(|key, modifier| match key {
        iced::keyboard::Key::Named(Named::Tab) => Some(if modifier.shift() {
            AppMessage::SelectPrev
        } else {
            AppMessage::SelectNext
        }),
        _ => None,
    })
}
