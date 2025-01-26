use reqwest::Response;
use serde::Deserialize;

use crate::{
    app::AppMessage,
    components::notification::{NotificationMessage, NotificationType},
};

pub async fn decode_error_response(response: Response) -> String {
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct ErrorResponse {
        error: String,
    }

    if response.status().is_success() {
        "".to_owned()
    } else {
        let error_response: Result<ErrorResponse, _> = response.json().await;
        match error_response {
            Ok(ok) => ok.error,
            Err(err) => {
                println!("error deserialising error response: {err}");
                "Unexpected error".to_owned()
            }
        }
    }
}

pub fn show_notification<T>(message: T, notification_type: NotificationType) -> AppMessage
where
    T: Into<String>,
{
    AppMessage::NotificationMessage(NotificationMessage::ShowNotification {
        message: message.into(),
        notification_type,
    })
}
