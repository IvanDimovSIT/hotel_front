use app::HotelApp;
use iced::Theme;

mod app;
mod screens;
mod services;

fn main() -> iced::Result {
    iced::application(HotelApp::title, HotelApp::update, HotelApp::view)
        .window_size((800.0, 600.0))
        .theme(|_| Theme::CatppuccinMacchiato)
        .run_with(HotelApp::new)
}

