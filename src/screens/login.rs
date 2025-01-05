use std::sync::{Arc, Mutex};

use iced::{
    widget::{button, column, text, text_input},
    Alignment::Center,
    Color,
    Length::Fill,
    Task,
};

use crate::{
    app::{AppMessage, GlobalState, Screen, ScreenType},
    services,
    styles::TEXT_BOX_WIDTH,
};

#[derive(Debug, Clone)]
pub enum LoginMessage {
    ChangeEmail(String),
    ChangePassword(String),
    Login,
}

pub struct LoginScreen {
    email: String,
    password: String,
    error: Arc<Mutex<String>>,
}
impl LoginScreen {
    pub fn new() -> Self {
        Self {
            email: "".to_owned(),
            password: "".to_owned(),
            error: Arc::new(Mutex::new("".to_owned())),
        }
    }
}
impl Screen for LoginScreen {
    fn update(
        &mut self,
        message: crate::app::AppMessage,
        global_state: Arc<Mutex<GlobalState>>,
    ) -> iced::Task<crate::app::AppMessage> {
        match message {
            AppMessage::LoginMessage(login_message) => match login_message {
                LoginMessage::ChangeEmail(email) => {
                    self.email = email;
                    Task::none()
                }
                LoginMessage::ChangePassword(password) => {
                    self.password = password;
                    Task::none()
                }
                LoginMessage::Login => {
                    println!(
                        "Logging in with email:'{}' and password '{}'",
                        self.email, self.password
                    );
                    let error = self.error.clone();
                    let global_state_copy = global_state.clone();
                    let email = self.email.clone();
                    let password = self.password.clone();
                    Task::perform(
                        async { services::login::login(email, password).await },
                        move |res| match res {
                            Ok(token) => {
                                println!("Set token: '{token}'");
                                global_state_copy.lock().unwrap().token = Some(token);
                                AppMessage::NavigateTo(ScreenType::Home)
                            }
                            Err(err) => {
                                println!("Error: {err}");
                                *error.lock().unwrap() = "Invalid username or password".to_owned();
                                AppMessage::None
                            }
                        },
                    )
                }
            },
            _ => Task::none(),
        }
    }

    fn view(&self, global_state: Arc<Mutex<GlobalState>>) -> iced::Element<crate::app::AppMessage> {
        column![
            text!("Login")
                .height(40)
                .size(30)
                .align_x(Center)
                .width(Fill),
            text_input("Email", &self.email)
                .on_input(|x| AppMessage::LoginMessage(LoginMessage::ChangeEmail(x)))
                .align_x(Center)
                .width(TEXT_BOX_WIDTH)
                .line_height(1.5),
            text_input("Password", &self.password)
                .on_input(|x| AppMessage::LoginMessage(LoginMessage::ChangePassword(x)))
                .align_x(Center)
                .secure(true)
                .width(TEXT_BOX_WIDTH)
                .line_height(1.5),
            text!("{}", self.error.lock().unwrap())
                .color(Color {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0
                })
                .size(18)
                .align_x(Center)
                .width(Fill),
            button("Log in")
                .on_press(AppMessage::LoginMessage(LoginMessage::Login))
                .height(30)
                .width(80)
        ]
        .spacing(20)
        .height(Fill)
        .align_x(Center)
        .into()
    }
}
