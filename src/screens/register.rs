use std::sync::{Arc, Mutex};

use iced::{
    widget::{button, column, text, text_input},
    Alignment::Center,
    Length::Fill,
    Task,
};
use uuid::Uuid;

use crate::{
    app::{AppMessage, GlobalState, Screen, ScreenType},
    components::{
        focus_chain::FocusChain,
        notification::NotificationType,
        text_box::text_box::{TextBox, TextElement},
    },
    constants::{MAX_EMAIL_LENGTH, MAX_PASSWORD_LENGTH},
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

const EMAIL_ID: &str = "Register Email";
const PASSWORD_ID: &str = "Register Password";
const PASSWORD_CONFIRM_ID: &str = "Register Password Confirm";

pub struct RegisterScreen {
    email: TextBox,
    password: TextBox,
    confirm_password: TextBox,
    error: String,
    focus_chain: FocusChain,
}
impl RegisterScreen {
    pub fn new() -> Self {
        Self {
            email: TextBox::new("", MAX_EMAIL_LENGTH),
            password: TextBox::new("", MAX_PASSWORD_LENGTH),
            confirm_password: TextBox::new("", MAX_PASSWORD_LENGTH),
            error: "".to_owned(),
            focus_chain: FocusChain::new(vec![EMAIL_ID, PASSWORD_ID, PASSWORD_CONFIRM_ID]),
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

        Task::perform(services::register::register(input), |res| match res {
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
        })
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
                    self.focus_chain.set_focus(Some(EMAIL_ID));
                    self.email.update(x);
                    Task::none()
                }
                RegisterMessage::ChangePassword(x) => {
                    self.focus_chain.set_focus(Some(PASSWORD_ID));
                    self.password.update(x);
                    Task::none()
                }
                RegisterMessage::ChangeConfirmPassword(x) => {
                    self.focus_chain.set_focus(Some(PASSWORD_CONFIRM_ID));
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
            AppMessage::SelectNext => {
                self.focus_chain.set_next();
                self.focus_chain.apply_focus()
            }
            AppMessage::SelectPrev => {
                self.focus_chain.set_prev();
                self.focus_chain.apply_focus()
            }
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
                .id(EMAIL_ID)
                .on_input(|x| AppMessage::RegisterMessage(RegisterMessage::ChangeEmail(x)))
                .on_submit(AppMessage::RegisterMessage(RegisterMessage::Register))
                .align_x(Center)
                .width(TEXT_BOX_WIDTH)
                .line_height(1.5),
            text_input("Password", self.password.get_text())
                .id(PASSWORD_ID)
                .on_input(|x| AppMessage::RegisterMessage(RegisterMessage::ChangePassword(x)))
                .on_submit(AppMessage::RegisterMessage(RegisterMessage::Register))
                .align_x(Center)
                .secure(true)
                .width(TEXT_BOX_WIDTH)
                .line_height(1.5),
            text_input("Confirm Password", self.confirm_password.get_text())
                .id(PASSWORD_CONFIRM_ID)
                .on_input(
                    |x| AppMessage::RegisterMessage(RegisterMessage::ChangeConfirmPassword(x))
                )
                .on_submit(AppMessage::RegisterMessage(RegisterMessage::Register))
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
