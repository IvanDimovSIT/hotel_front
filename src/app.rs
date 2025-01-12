use std::sync::{Arc, Mutex};

use iced::widget::{button, column, row};
use iced::{Element, Task};

use crate::components::validator::Validator;
use crate::screens::add_room::{AddRoomMessage, AddRoomScreen};
use crate::screens::home::HomeScreen;
use crate::screens::login::{LoginMessage, LoginScreen};
use crate::security::{JwtToken, Role};
use crate::styles::NAVIGATION_BUTTON_WIDTH;

#[derive(Debug, Default)]
pub struct GlobalState {
    pub token: Option<JwtToken>,
    pub validator: Validator,
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
    AddRoomMessage(AddRoomMessage),
}

#[derive(Debug, Clone)]
pub enum ScreenType {
    Home,
    Login,
    AddRoom,
}
impl ScreenType {
    fn create_screen(&self) -> Box<dyn Screen> {
        match self {
            ScreenType::Home => Box::new(HomeScreen::new()),
            ScreenType::Login => Box::new(LoginScreen::new()),
            ScreenType::AddRoom => Box::new(AddRoomScreen::new()),
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
            _ => match &self.global_state.lock().unwrap().token {
                Some(some) => match some.role {
                    Role::User => self.view_user(),
                    Role::Admin => self.view_admin(),
                },
                None => self.view_user(),
            },
        }
    }

    fn view_admin(&self) -> Element<AppMessage> {
        row![
            column![
                button("Add Room")
                    .on_press(AppMessage::NavigateTo(ScreenType::AddRoom))
                    .width(NAVIGATION_BUTTON_WIDTH),
                button("Placeholder2").width(NAVIGATION_BUTTON_WIDTH),
                button("Placeholder3").width(NAVIGATION_BUTTON_WIDTH),
            ],
            self.current_screen.view(self.global_state.clone())
        ]
        .into()
    }

    fn view_user(&self) -> Element<AppMessage> {
        row![
            column![
                button("Placeholder1").width(NAVIGATION_BUTTON_WIDTH),
                button("Placeholder2").width(NAVIGATION_BUTTON_WIDTH),
                button("Placeholder3").width(NAVIGATION_BUTTON_WIDTH),
            ],
            self.current_screen.view(self.global_state.clone())
        ]
        .into()
    }
}
