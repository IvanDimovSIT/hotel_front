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
    components::{
        notification::NotificationType,
        text_box::text_box::{TextBox, TextElement},
    },
    constants::{MAX_EMAIL_LENGTH, MAX_PASSWORD_LENGTH},
    services::{self, send_otp::SendOtpResult},
    styles::{ERROR_COLOR, FORM_PADDING, FORM_SPACING, TEXT_BOX_WIDTH},
    utils::show_notification,
};

#[derive(Debug, Clone)]
pub enum ResetPasswordMessage {
    ChangeOtp(String),
    ChangePassword(String),
    ChangeConfirmPassword(String),
    ErrorResendingCode(String),
    ResetPassword,
    ResendCode,
    CodeResent { email: String },
}

pub struct ResetPasswordScreen {
    otp_input: TextBox,
    new_password_input: TextBox,
    confirm_new_password_input: TextBox,
    error: String,
}
impl ResetPasswordScreen {
    pub fn new() -> Self {
        Self {
            otp_input: TextBox::new("", 8),
            new_password_input: TextBox::new("", MAX_PASSWORD_LENGTH),
            confirm_new_password_input: TextBox::new("", MAX_PASSWORD_LENGTH),
            error: "".to_owned(),
        }
    }

    fn resend_code(&mut self, global_state: Arc<Mutex<GlobalState>>) -> Task<AppMessage> {
        let email = if let Some(some) = &global_state.lock().unwrap().email {
            some.to_owned()
        } else {
            return Task::done(AppMessage::NavigateTo(ScreenType::Login)).chain(Task::done(
                show_notification("Unexpected error", NotificationType::Error),
            ));
        };
        let email_copy = email.clone();

        Task::perform(
            async { services::send_otp::send_otp(email).await },
            move |res| match res {
                Ok(SendOtpResult::Success) => {
                    AppMessage::ResetPasswordMessage(ResetPasswordMessage::CodeResent {
                        email: email_copy.clone(),
                    })
                }
                Ok(SendOtpResult::BadRequest(err)) => {
                    AppMessage::ResetPasswordMessage(ResetPasswordMessage::ErrorResendingCode(err))
                }
                Err(err) => {
                    println!("Error sending otp: {err}");
                    show_notification("Unexpected error", NotificationType::Error)
                }
            },
        )
    }

    fn reset_password(&mut self, global_state: Arc<Mutex<GlobalState>>) -> Task<AppMessage> {
        let password = self.new_password_input.get_text().to_owned();
        if let Err(err) = global_state
            .lock()
            .unwrap()
            .validator
            .validate_password(&password)
        {
            self.error = err;
            return Task::none();
        }
        if self.confirm_new_password_input.get_text() != password {
            self.error = "Passwords don't match".to_owned();
            return Task::none();
        }

        let email = if let Some(some) = &global_state.lock().unwrap().email {
            some.to_owned()
        } else {
            return Task::done(AppMessage::NavigateTo(ScreenType::Login)).chain(Task::done(
                show_notification("Unexpected error", NotificationType::Error),
            ));
        };
        let otp = self.otp_input.get_text().to_owned();
        if otp.trim().is_empty() {
            self.error = format!(
                "Enter code sent to '{}'",
                global_state
                    .lock()
                    .unwrap()
                    .email
                    .as_ref()
                    .map_or("", |x| x)
            );
            return Task::none();
        }

        todo!("Send reset password request with email '{email}', otp '{otp}' and password '{password}' ")
    }
}
impl Screen for ResetPasswordScreen {
    fn update(
        &mut self,
        message: AppMessage,
        global_state: Arc<Mutex<GlobalState>>,
    ) -> Task<AppMessage> {
        match message {
            AppMessage::ResetPasswordMessage(m) => match m {
                ResetPasswordMessage::ChangeOtp(otp) => {
                    self.otp_input.update(otp);
                    Task::none()
                }
                ResetPasswordMessage::ChangePassword(password) => {
                    self.new_password_input.update(password);
                    Task::none()
                }
                ResetPasswordMessage::ChangeConfirmPassword(confirm_password) => {
                    self.confirm_new_password_input.update(confirm_password);
                    Task::none()
                }
                ResetPasswordMessage::ResendCode => self.resend_code(global_state),
                ResetPasswordMessage::ResetPassword => self.reset_password(global_state),
                ResetPasswordMessage::ErrorResendingCode(err) => {
                    self.error = err;
                    Task::none()
                }
                ResetPasswordMessage::CodeResent { email } => {
                    self.error = "".to_owned();
                    Task::done(show_notification(
                        format!("Code sent to '{email}'"),
                        NotificationType::Information,
                    ))
                }
            },
            _ => Task::none(),
        }
    }

    fn view(&self, _global_state: Arc<Mutex<GlobalState>>) -> Element<AppMessage> {
        column![
            text!("Reset Password")
                .height(40)
                .size(30)
                .align_x(Center)
                .width(Fill),
            text_input("Code", self.otp_input.get_text())
                .on_input(|x| AppMessage::ResetPasswordMessage(ResetPasswordMessage::ChangeOtp(x)))
                .align_x(Center)
                .width(TEXT_BOX_WIDTH)
                .line_height(1.5),
            text_input("Password", self.new_password_input.get_text())
                .on_input(|x| AppMessage::ResetPasswordMessage(
                    ResetPasswordMessage::ChangePassword(x)
                ))
                .align_x(Center)
                .secure(true)
                .width(TEXT_BOX_WIDTH)
                .line_height(1.5),
            text_input(
                "Confirm Password",
                self.confirm_new_password_input.get_text()
            )
            .on_input(|x| AppMessage::ResetPasswordMessage(
                ResetPasswordMessage::ChangeConfirmPassword(x)
            ))
            .align_x(Center)
            .secure(true)
            .width(TEXT_BOX_WIDTH)
            .line_height(1.5),
            text!("{}", self.error)
                .color(ERROR_COLOR)
                .size(18)
                .align_x(Center)
                .width(Fill),
            button("Change password")
                .on_press(AppMessage::ResetPasswordMessage(
                    ResetPasswordMessage::ResetPassword
                ))
                .height(30)
                .width(140),
            button("Resend code")
                .on_press(AppMessage::ResetPasswordMessage(
                    ResetPasswordMessage::ResendCode
                ))
                .height(30)
                .width(140),
            button("Back to login")
                .on_press(AppMessage::NavigateTo(ScreenType::Login))
                .height(30)
                .width(140)
        ]
        .spacing(FORM_SPACING)
        .padding(FORM_PADDING)
        .height(Fill)
        .align_x(Center)
        .into()
    }
}
