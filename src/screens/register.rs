use std::sync::{Arc, Mutex};

use iced::{
    widget::{button, column, text, text_input},
    Alignment::Center,
    Element,
    Length::Fill,
    Task,
};
use uuid::Uuid;

use crate::{
    app::{AppMessage, GlobalState, Screen, ScreenType},
    components::{
        notification::{NotificationMessage, NotificationType},
        text_box::text_box::{TextBox, TextElement},
    },
    services::{
        self,
        register::{RegisterInput, RegisterResult},
    },
    styles::{ERROR_COLOR, FORM_PADDING, FORM_SPACING, TEXT_BOX_WIDTH},
    utils::show_notification,
};

#[derive(Debug, Clone)]
pub enum RegisterMessage {
    ChangeEmail(String),
    ChangePassword(String),
    ChangeConfirmPassword(String),
    Register,
    Registered(Uuid),
    UpdateError(String),
}

pub struct RegisterScreen {
    email: TextBox,
    password: TextBox,
    confirm_password: TextBox,
    error: String,
}
impl RegisterScreen {
    pub fn new() -> Self {
        Self {
            email: TextBox::new("", 40),
            password: TextBox::new("", 24),
            confirm_password: TextBox::new("", 24),
            error: "".to_owned(),
        }
    }

    fn validate_input(&mut self, global_state: Arc<Mutex<GlobalState>>) -> Result<(), String> {
        let lock = global_state.lock().unwrap();
        lock.validator.validate_email(self.email.get_text())?;
        lock.validator.validate_password(self.password.get_text())?;
        if self.password.get_text() != self.confirm_password.get_text() {
            Err("Passwords don't match".to_owned())
        } else {
            Ok(())
        }
    }

    fn register(&mut self, global_state: Arc<Mutex<GlobalState>>) -> Task<AppMessage> {
        if let Err(err) = self.validate_input(global_state) {
            self.error = err;
            return Task::none();
        }

        let input = RegisterInput {
            email: self.email.get_text().to_owned(),
            password: self.password.get_text().to_owned(),
        };

        Task::perform(
            async { services::register::register(input).await },
            |res| match res {
                Ok(RegisterResult::Registered(uid)) => {
                    AppMessage::RegisterMessage(RegisterMessage::Registered(uid))
                }
                Ok(RegisterResult::BadRequest(err)) => {
                    AppMessage::RegisterMessage(RegisterMessage::UpdateError(err))
                }
                Err(err) => {
                    println!("Error registering: '{err}'");
                    show_notification("Unexpected error", NotificationType::Error)
                }
            },
        )
    }
}
impl Screen for RegisterScreen {
    fn update(
        &mut self,
        message: AppMessage,
        global_state: Arc<Mutex<GlobalState>>,
    ) -> Task<AppMessage> {
        match message {
            AppMessage::RegisterMessage(register_message) => match register_message {
                RegisterMessage::ChangeEmail(x) => {
                    self.email.update(x);
                    Task::none()
                }
                RegisterMessage::ChangePassword(x) => {
                    self.password.update(x);
                    Task::none()
                }
                RegisterMessage::ChangeConfirmPassword(x) => {
                    self.confirm_password.update(x);
                    Task::none()
                }
                RegisterMessage::Register => self.register(global_state),
                RegisterMessage::Registered(user_id) => {
                    println!("Registered user with id '{user_id}'");
                    Task::done(AppMessage::NavigateTo(ScreenType::Login)).chain(Task::done(
                        show_notification("Registered successfully", NotificationType::Success),
                    ))
                }
                RegisterMessage::UpdateError(err) => {
                    self.error = err;
                    Task::none()
                }
            },
            _ => Task::none(),
        }
    }

    fn view(
        &self,
        _global_state: Arc<Mutex<crate::app::GlobalState>>,
    ) -> iced::Element<crate::app::AppMessage> {
        column![
            text!("Register")
                .height(40)
                .size(30)
                .align_x(Center)
                .width(Fill),
            text_input("Email", self.email.get_text())
                .on_input(|x| AppMessage::RegisterMessage(RegisterMessage::ChangeEmail(x)))
                .align_x(Center)
                .width(TEXT_BOX_WIDTH)
                .line_height(1.5),
            text_input("Password", self.password.get_text())
                .on_input(|x| AppMessage::RegisterMessage(RegisterMessage::ChangePassword(x)))
                .align_x(Center)
                .secure(true)
                .width(TEXT_BOX_WIDTH)
                .line_height(1.5),
            text_input("Confirm Password", self.confirm_password.get_text())
                .on_input(
                    |x| AppMessage::RegisterMessage(RegisterMessage::ChangeConfirmPassword(x))
                )
                .align_x(Center)
                .secure(true)
                .width(TEXT_BOX_WIDTH)
                .line_height(1.5),
            text!("{}", self.error)
                .color(ERROR_COLOR)
                .size(18)
                .align_x(Center)
                .width(Fill),
            button("Register")
                .on_press(AppMessage::RegisterMessage(RegisterMessage::Register))
                .height(30)
                .width(80),
            button("Log in")
                .on_press(AppMessage::NavigateTo(ScreenType::Login))
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
