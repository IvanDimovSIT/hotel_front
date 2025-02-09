use std::sync::{Arc, Mutex};

use iced::{
    widget::{button, column, text, text_input},
    Alignment::Center,
    Element,
    Length::Fill,
    Task,
};

use crate::{
    app::{AppMessage, GlobalState, Screen, ScreenType},
    components::text_box::text_box::{TextBox, TextElement},
    services,
    styles::{ERROR_COLOR, FORM_PADDING, FORM_SPACING, TEXT_BOX_WIDTH},
};

#[derive(Debug, Clone)]
pub enum LoginMessage {
    ChangeEmail(String),
    ChangePassword(String),
    Login,
}

pub struct LoginScreen {
    email: TextBox,
    password: TextBox,
    error: Arc<Mutex<String>>,
}
impl LoginScreen {
    pub fn new() -> Self {
        Self {
            email: TextBox::new("", 40),
            password: TextBox::new("", 24),
            error: Arc::new(Mutex::new("".to_owned())),
        }
    }
}
impl Screen for LoginScreen {
    fn update(
        &mut self,
        message: AppMessage,
        global_state: Arc<Mutex<GlobalState>>,
    ) -> Task<AppMessage> {
        match message {
            AppMessage::LoginMessage(login_message) => match login_message {
                LoginMessage::ChangeEmail(email) => {
                    self.email.update(email);
                    Task::none()
                }
                LoginMessage::ChangePassword(password) => {
                    self.password.update(password);
                    Task::none()
                }
                LoginMessage::Login => {
                    let error = self.error.clone();
                    let global_state_input = global_state.clone();
                    let global_state_copy = global_state.clone();
                    let email = self.email.get_text().to_owned();
                    let password = self.password.get_text().to_owned();
                    Task::perform(
                        async { services::login::login(global_state_input, email, password).await },
                        move |res| match res {
                            Ok(token) => {
                                println!("Set token: '{token:?}'");
                                global_state_copy.lock().unwrap().token = Some(token);
                                AppMessage::NavigateTo(ScreenType::Home)
                            }
                            Err(err) => {
                                println!("Error: {err}");
                                *error.lock().unwrap() = err;
                                AppMessage::None
                            }
                        },
                    )
                }
            },
            _ => Task::none(),
        }
    }

    fn view(&self, _global_state: Arc<Mutex<GlobalState>>) -> Element<crate::app::AppMessage> {
        column![
            text!("Login")
                .height(40)
                .size(30)
                .align_x(Center)
                .width(Fill),
            text_input("Email", self.email.get_text())
                .on_input(|x| AppMessage::LoginMessage(LoginMessage::ChangeEmail(x)))
                .align_x(Center)
                .width(TEXT_BOX_WIDTH)
                .line_height(1.5),
            text_input("Password", self.password.get_text())
                .on_input(|x| AppMessage::LoginMessage(LoginMessage::ChangePassword(x)))
                .align_x(Center)
                .secure(true)
                .width(TEXT_BOX_WIDTH)
                .line_height(1.5),
            text!("{}", self.error.lock().unwrap())
                .color(ERROR_COLOR)
                .size(18)
                .align_x(Center)
                .width(Fill),
            button("Log in")
                .on_press(AppMessage::LoginMessage(LoginMessage::Login))
                .height(30)
                .width(80)
        ]
        .spacing(FORM_SPACING)
        .padding(FORM_PADDING)
        .height(Fill)
        .align_x(Center)
        .into()
    }
}
