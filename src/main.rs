use app::HotelApp;
use iced_aw::iced_fonts::REQUIRED_FONT_BYTES;
use styles::MAIN_THEME;
use subscriptions::refresh_token_subscription::refresh_token_subscription;

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
        .window_size((800.0, 600.0))
        .theme(|_| MAIN_THEME)
        .font(REQUIRED_FONT_BYTES)
        .run_with(HotelApp::new)
}
