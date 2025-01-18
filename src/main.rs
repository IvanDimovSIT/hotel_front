use app::HotelApp;
use styles::MAIN_THEME;

mod app;
mod components;
mod constants;
mod screens;
mod security;
mod services;
mod styles;
mod utils;

fn main() -> iced::Result {
    iced::application(HotelApp::title, HotelApp::update, HotelApp::view)
        .window_size((800.0, 600.0))
        .theme(|_| MAIN_THEME)
        .run_with(HotelApp::new)
}
