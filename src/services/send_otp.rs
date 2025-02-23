use std::error::Error;

use ::serde::Serialize;
use reqwest::Response;
use serde_json::to_string;

use crate::{
    constants::{BASE_URL, SEND_OTP_PATH},
    utils::decode_error_response,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct SendOtpInput {
    email: String,
}

pub enum SendOtpResult {
    Success,
    BadRequest(String),
}

async fn map_result(http_result: Result<Response, impl Error>) -> Result<SendOtpResult, String> {
    match http_result {
        Ok(response) => {
            if response.status().is_success() {
                Ok(SendOtpResult::Success)
            } else if response.status().is_client_error() {
                Ok(SendOtpResult::BadRequest(
                    decode_error_response(response).await,
                ))
            } else {
                Err(decode_error_response(response).await)
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

pub async fn send_otp(email: String) -> Result<SendOtpResult, String> {
    let url = BASE_URL.to_owned() + SEND_OTP_PATH;
    let client = reqwest::Client::new();
    let input = SendOtpInput { email };
    println!("POST {url}");

    let body = match to_string(&input) {
        Ok(ok) => ok,
        Err(err) => {
            return Err(err.to_string());
        }
    };

    let result = client
        .post(url)
        .body(body)
        .header("Content-Type", "application/json")
        .send()
        .await;

    map_result(result).await
}
