use std::sync::{Arc, Mutex};

use iced::{
    widget::{button, column, row, text, text_input},
    Alignment::Center,
    Element,
    Length::Fill,
    Task,
};

use crate::{
    app::{AppMessage, GlobalState, Screen, ScreenType},
    components::{
        focus_chain::FocusChain,
        notification::NotificationType,
        text_box::text_box::{TextBox, TextElement},
    },
    constants::{MAX_EMAIL_LENGTH, MAX_PASSWORD_LENGTH},
    services::{self, send_otp::SendOtpResult},
    styles::{ERROR_COLOR, FORM_PADDING, FORM_SPACING, TEXT_BOX_WIDTH},
    utils::show_notification,
};

#[derive(Debug, Clone)]
pub enum LoginMessage {
    ChangeEmail(String),
    ChangePassword(String),
    ResetPassword,
    OtpSent,
    Login,
    SetError(String),
}

const EMAIL_ID: &str = "Login Email";
const PASSWORD_ID: &str = "Login Password";

pub struct LoginScreen {
    email: TextBox,
    password: TextBox,
    error: String,
    focus_chain: FocusChain,
}
impl LoginScreen {
    pub fn new() -> Self {
        Self {
            email: TextBox::new("", MAX_EMAIL_LENGTH),
            password: TextBox::new("", MAX_PASSWORD_LENGTH),
            error: "".to_owned(),
            focus_chain: FocusChain::new(vec![EMAIL_ID, PASSWORD_ID]),
        }
    }

    fn reset_password(&mut self, global_state: Arc<Mutex<GlobalState>>) -> Task<AppMessage> {
        if global_state
            .lock()
            .unwrap()
            .validator
            .validate_email(self.email.get_text())
            .is_err()
        {
            self.error = "Enter a valid email to reset password".to_owned();
            return Task::none();
        }

        let email_input = self.email.get_text().to_owned();
        let email_copy = email_input.clone();

        Task::perform(
            async { services::send_otp::send_otp(email_input).await },
            move |res| match res {
                Ok(SendOtpResult::Success) => {
                    global_state.lock().unwrap().email = Some(email_copy.clone());
                    AppMessage::LoginMessage(LoginMessage::OtpSent)
                }
                Ok(SendOtpResult::BadRequest(err)) => {
                    AppMessage::LoginMessage(LoginMessage::SetError(err))
                }
                Err(err) => {
                    println!("Error sending otp: {err}");
                    AppMessage::LoginMessage(LoginMessage::SetError(err))
                }
            },
        )
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
                    self.focus_chain.set_focus(Some(EMAIL_ID));
                    self.email.update(email);
                    Task::none()
                }
                LoginMessage::ChangePassword(password) => {
                    self.focus_chain.set_focus(Some(PASSWORD_ID));
                    self.password.update(password);
                    Task::none()
                }
                LoginMessage::Login => {
                    let global_state_input = global_state.clone();
                    let global_state_copy = global_state.clone();
                    let email = self.email.get_text().to_owned();
                    let password = self.password.get_text().to_owned();
                    Task::perform(
                        services::login::login(global_state_input, email, password),
                        move |res| match res {
                            Ok(token) => {
                                println!("Set token: '{token:?}'");
                                global_state_copy.lock().unwrap().token = Some(token);
                                AppMessage::NavigateTo(ScreenType::Home)
                            }
                            Err(err) => {
                                println!("Error: {err}");
                                AppMessage::LoginMessage(LoginMessage::SetError(err))
                            }
                        },
                    )
                }
                LoginMessage::ResetPassword => self.reset_password(global_state),
                LoginMessage::OtpSent => Task::done(show_notification(
                    "Code sent, check your email",
                    NotificationType::Information,
                ))
                .chain(Task::done(AppMessage::NavigateTo(
                    ScreenType::ResetPassword,
                ))),
                LoginMessage::SetError(err) => {
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

    fn view(&self, _global_state: Arc<Mutex<GlobalState>>) -> Element<crate::app::AppMessage> {
        column![
            text!("Login")
                .height(40)
                .size(30)
                .align_x(Center)
                .width(Fill),
            text_input("Email", self.email.get_text())
                .id(EMAIL_ID)
                .on_input(|x| AppMessage::LoginMessage(LoginMessage::ChangeEmail(x)))
                .on_submit(AppMessage::LoginMessage(LoginMessage::Login))
                .align_x(Center)
                .width(TEXT_BOX_WIDTH)
                .line_height(1.5),
            text_input("Password", self.password.get_text())
                .id(PASSWORD_ID)
                .on_input(|x| AppMessage::LoginMessage(LoginMessage::ChangePassword(x)))
                .on_submit(AppMessage::LoginMessage(LoginMessage::Login))
                .align_x(Center)
                .secure(true)
                .width(TEXT_BOX_WIDTH)
                .line_height(1.5),
            text!("{}", self.error)
                .color(ERROR_COLOR)
                .size(18)
                .align_x(Center)
                .width(Fill),
            button("Log in")
                .on_press(AppMessage::LoginMessage(LoginMessage::Login))
                .height(30)
                .width(80),
            row![
                button("Register")
                    .on_press(AppMessage::NavigateTo(ScreenType::Register))
                    .height(30)
                    .width(140),
                button("Reset password")
                    .on_press(AppMessage::LoginMessage(LoginMessage::ResetPassword))
                    .height(30)
                    .width(140),
            ]
            .spacing(10)
        ]
        .spacing(FORM_SPACING)
        .padding(FORM_PADDING)
        .height(Fill)
        .align_x(Center)
        .into()
    }
}
