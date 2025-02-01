use std::sync::{Arc, Mutex};

use iced::widget::column;
use iced::{Element, Task};

use crate::components::navigation_bar::{view_admin, view_user};
use crate::components::notification::{Notification, NotificationMessage, NotificationType};
use crate::components::validator::Validator;
use crate::screens::add_guest::{AddGuestMessage, AddGuestScreen};
use crate::screens::add_room::{AddRoomMessage, AddRoomScreen};
use crate::screens::home::HomeScreen;
use crate::screens::login::{LoginMessage, LoginScreen};
use crate::security::{JwtToken, Role};
use crate::services;
use crate::utils::show_notification;

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
    RefreshToken,
    TokenExpired,
    NotificationMessage(NotificationMessage),
    NavigateTo(ScreenType),
    LoginMessage(LoginMessage),
    AddRoomMessage(AddRoomMessage),
    AddGuestMessage(AddGuestMessage),
}

#[derive(Debug, Clone)]
pub enum ScreenType {
    Home,
    Login,
    AddRoom,
    AddGuest,
}
impl ScreenType {
    fn create_screen(&self) -> Box<dyn Screen> {
        match self {
            ScreenType::Home => Box::new(HomeScreen::new()),
            ScreenType::Login => Box::new(LoginScreen::new()),
            ScreenType::AddRoom => Box::new(AddRoomScreen::new()),
            ScreenType::AddGuest => Box::new(AddGuestScreen::new()),
        }
    }
}

pub struct HotelApp {
    current_screen: Box<dyn Screen>,
    screen_type: ScreenType,
    notification: Notification,
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
        let notification = Notification::new();

        (
            Self {
                current_screen,
                screen_type,
                global_state,
                notification,
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
            AppMessage::TokenExpired => Task::done(show_notification(
                "Login session expired",
                NotificationType::Error,
            ))
            .chain(Task::done(AppMessage::NavigateTo(ScreenType::Login))),
            AppMessage::RefreshToken => self.refresh_token(),
            AppMessage::None => Task::none(),
            AppMessage::NotificationMessage(notification_message) => {
                self.notification.update(notification_message)
            }
            _ => self
                .current_screen
                .update(message, self.global_state.clone()),
        }
    }

    pub fn view(&self) -> Element<AppMessage> {
        column![match self.screen_type {
            ScreenType::Login => self.current_screen.view(self.global_state.clone()),
            _ => match &self.global_state.lock().unwrap().token {
                Some(some) => match some.role {
                    Role::User => view_user(self.global_state.clone(), &*self.current_screen),
                    Role::Admin => view_admin(self.global_state.clone(), &*self.current_screen),
                },
                None => view_user(self.global_state.clone(), &*self.current_screen),
            },
        }]
        .push_maybe(self.notification.view())
        .into()
    }

    fn refresh_token(&self) -> Task<AppMessage> {
        if self.global_state.lock().unwrap().token.is_some() {
            let global_state_copy = self.global_state.clone();
            let token_string = global_state_copy
                .lock()
                .unwrap()
                .token
                .as_ref()
                .unwrap()
                .token_string
                .clone();

            Task::perform(
                async { services::refresh_token::refresh_token(token_string).await },
                move |res| {
                    match res {
                        Ok(ok) => {
                            println!("refreshed token: {}", ok.token_string);
                            global_state_copy.lock().unwrap().token = Some(ok);
                        }
                        Err(err) => println!("Error refreshing token: {err}"),
                    }
                    AppMessage::None
                },
            )
        } else {
            Task::none()
        }
    }
}
