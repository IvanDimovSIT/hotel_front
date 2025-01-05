use std::sync::{Arc, Mutex};

use iced::widget::{button, column, row};
use iced::window;
use iced::{Center, Element, Fill, Font, Subscription, Task};

use crate::screens::home::HomeScreen;
use crate::screens::login::{LoginMessage, LoginScreen};

#[derive(Debug, Default)]
pub struct GlobalState {
    pub token: Option<String>,
}

pub trait Screen {
    fn update(
        &mut self,
        message: AppMessage,
        global_state: Arc<Mutex<GlobalState>>,
    ) -> Task<AppMessage>;
    fn view(&self, global_state: Arc<Mutex<GlobalState>>) -> Element<AppMessage>;
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    None,
    NavigateTo(ScreenType),
    LoginMessage(LoginMessage),
}

#[derive(Debug, Clone)]
pub enum ScreenType {
    Home,
    Login,
}
impl ScreenType {
    fn create_screen(&self) -> Box<dyn Screen> {
        match self {
            ScreenType::Home => Box::new(HomeScreen::new()),
            ScreenType::Login => Box::new(LoginScreen::new()),
        }
    }
}

pub struct HotelApp {
    current_screen: Box<dyn Screen>,
    screen_type: ScreenType,
    global_state: Arc<Mutex<GlobalState>>,
}
impl HotelApp {
    fn navigate_to(&mut self, screen: &ScreenType) -> Task<AppMessage> {
        self.screen_type = screen.clone();
        self.current_screen = screen.create_screen();
        Task::none()
    }

    pub fn new() -> (Self, Task<AppMessage>) {
        let global_state = Arc::new(Mutex::new(GlobalState::default()));
        let screen_type = ScreenType::Login;
        let current_screen = screen_type.create_screen();

        (
            Self {
                current_screen,
                screen_type,
                global_state,
            },
            Task::none(),
        )
    }

    pub fn title(&self) -> String {
        "Hotel".to_owned()
    }

    pub fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        match message {
            AppMessage::NavigateTo(screen_type) => self.navigate_to(&screen_type),
            AppMessage::None => Task::none(),
            _ => self
                .current_screen
                .update(message, self.global_state.clone()),
        }
    }

    pub fn view(&self) -> Element<AppMessage> {
        match self.screen_type {
            ScreenType::Login => self.current_screen.view(self.global_state.clone()),
            _ => row![
                column![
                    button("Placeholder1"),
                    button("Placeholder2"),
                    button("Placeholder3"),
                ],
                self.current_screen.view(self.global_state.clone())
            ]
            .into(),
        }
    }
}
