use reqwest::{Response, StatusCode};
use serde::Deserialize;

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
