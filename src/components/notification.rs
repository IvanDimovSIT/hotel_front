use std::time::Duration;

use iced::{
    theme::palette,
    widget::{container::Style, text, Container},
    Alignment::Center,
    Background, Color, Element,
    Length::Fill,
    Task, Theme,
};
use tokio::time::sleep;

use crate::app::AppMessage;

const NOTIFICATION_TIME: u64 = 4;
const NOTIFICATION_TEXT_SIZE: f32 = 21.0;
const NOTIFICAION_HEIGHT: f32 = 30.0;

#[derive(Debug, Clone, Copy)]
pub enum NotificationType {
    Information,
    Success,
    Error,
}

#[derive(Debug, Clone)]
pub enum NotificationMessage {
    ShowNotification {
        message: String,
        notification_type: NotificationType,
    },
    ClearNotification,
}

#[derive(Debug)]
pub struct Notification {
    display: bool,
    message: String,
    notification_type: NotificationType,
}
impl Notification {
    pub fn new() -> Self {
        Self {
            display: false,
            message: "Hello".to_owned(),
            notification_type: NotificationType::Information,
        }
    }

    fn get_style(&self) -> impl Fn(&Theme) -> Style {
        let notification_type = self.notification_type.clone();
        move |theme| {
            let palette = theme.palette();
            let background = Some(match notification_type {
                NotificationType::Information => Background::Color(palette.primary),
                NotificationType::Success => Background::Color(palette.success),
                NotificationType::Error => Background::Color(palette.danger),
            });

            Style {
                text_color: Some(palette.text),
                background,
                ..Default::default()
            }
        }
    }

    pub fn update(&mut self, message: NotificationMessage) -> Task<AppMessage> {
        match message {
            NotificationMessage::ShowNotification {
                message,
                notification_type,
            } => {
                self.display = true;
                self.message = message;
                self.notification_type = notification_type;
                Task::perform(sleep(Duration::from_secs(NOTIFICATION_TIME)), |_| {
                    AppMessage::NotificationMessage(NotificationMessage::ClearNotification)
                })
            }
            NotificationMessage::ClearNotification => {
                self.display = false;
                Task::none()
            }
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Option<Element<AppMessage>> {
        if !self.display {
            return None;
        }

        let contents = text!("{}", self.message)
            .align_x(Center)
            .size(NOTIFICATION_TEXT_SIZE);

        Some(
            Container::new(contents)
                .width(Fill)
                .align_x(Center)
                .height(NOTIFICAION_HEIGHT)
                .style(self.get_style())
                .into(),
        )
    }
}
impl Default for Notification {
    fn default() -> Self {
        Self::new()
    }
}
