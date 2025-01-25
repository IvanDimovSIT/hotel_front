use std::time::Duration;

use iced::{time::every, Subscription};

use crate::{
    app::{AppMessage, HotelApp},
    constants::REFRESH_TOKEN_PERIOD,
};

pub fn refresh_token_subscription(_hotel_app: &HotelApp) -> Subscription<AppMessage> {
    every(Duration::from_secs(REFRESH_TOKEN_PERIOD)).map(|_| AppMessage::RefreshToken)
}
