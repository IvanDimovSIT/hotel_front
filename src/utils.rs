use iced_aw::date_picker::Date;
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
        println!("Decoding error from 2xx response");
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

pub fn string_to_date(date_string: &str) -> Date {
    const DATE_COMPONENTS: usize = 3;
    const YEAR_INDEX: usize = 0;
    const MONTH_INDEX: usize = 1;
    const DAY_INDEX: usize = 2;
    let parts: Vec<_> = date_string.split("-").collect();

    if parts.len() != DATE_COMPONENTS {
        println!("Invalid date: '{date_string}'");
        return Date::default();
    }

    let year = parts[YEAR_INDEX].parse().unwrap_or_default();

    let month = parts[MONTH_INDEX].parse().unwrap_or_default();

    let day = parts[DAY_INDEX].parse().unwrap_or_default();

    Date::from_ymd(year, month, day)
}
