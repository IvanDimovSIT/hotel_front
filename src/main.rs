use app::HotelApp;
use constants::DEFAULT_WINDOW_SIZE;
use iced_aw::iced_fonts::REQUIRED_FONT_BYTES;
use styles::MAIN_THEME;
use subscriptions::{
    refresh_token_subscription::refresh_token_subscription,
    select_text_input_subscription::select_text_input_subscription,
};

mod app;
mod components;
mod constants;
mod model;
mod screens;
mod security;
mod services;
mod styles;
mod subscriptions;
mod utils;

fn main() -> iced::Result {
    iced::application(HotelApp::title, HotelApp::update, HotelApp::view)
        .subscription(refresh_token_subscription)
        .subscription(select_text_input_subscription)
        .window_size(DEFAULT_WINDOW_SIZE)
        .theme(|_| MAIN_THEME)
        .font(REQUIRED_FONT_BYTES)
        .run_with(HotelApp::new)
}
