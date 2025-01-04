use iced::{widget::{button, column, text, text_input}, Alignment::Center, Length::Fill, Task};

use crate::app::{AppMessage, GlobalState, Screen, ScreenType};

#[derive(Debug, Clone)]
pub enum LoginMessage{
    ChangeEmail(String),
    ChangePassword(String),
    Login,
}


pub struct LoginScreen {
    email: String,
    password: String
}
impl LoginScreen {
    pub fn new() -> Self {
        Self {
            email: "".to_owned(),
            password: "".to_owned()
        }
    }
}
impl Screen for LoginScreen {
    fn update(&mut self, message: crate::app::AppMessage, global_state: &mut GlobalState) -> iced::Task<crate::app::AppMessage> {        
        match message {
            AppMessage::LoginMessage(login_message) => match login_message {
                LoginMessage::ChangeEmail(email) => {
                    self.email = email;
                    Task::none()
                },
                LoginMessage::ChangePassword(password) => {
                    self.password = password;
                    Task::none()
                },
                LoginMessage::Login => {
                    println!("Logging in with email:'{}' and password '{}'", self.email, self.password);

                    Task::perform(async {}, |_| AppMessage::NavigateTo(ScreenType::Home))
                },
            },
            _ => Task::none(),
        }
    }

    fn view(&self, global_state: &GlobalState) -> iced::Element<crate::app::AppMessage> {
        column![
            text!("Login")
                .height(40)
                .size(30)
                .align_x(Center)
                .width(Fill),
            text_input("Email", &self.email)
                .on_input(|x| AppMessage::LoginMessage(LoginMessage::ChangeEmail(x)))
                .align_x(Center)
                .width(250)
                .line_height(1.5),
            text_input("Password", &self.password)
                .on_input(|x| AppMessage::LoginMessage(LoginMessage::ChangePassword(x)))
                .align_x(Center)
                .secure(true)
                .width(250)
                .line_height(1.5),
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