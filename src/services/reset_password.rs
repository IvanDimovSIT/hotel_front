use std::error::Error;

use ::serde::Serialize;
use reqwest::{header, Response};
use serde_json::to_string;

use crate::{
    constants::{BASE_URL, RESET_PASSWORD_PATH},
    utils::decode_error_response,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetPasswordInput {
    pub email: String,
    pub otp: String,
    pub new_password: String,
}

pub enum ResetPasswordResult {
    PasswordReset,
    BadRequest(String),
}

async fn map_result(result: Result<Response, impl Error>) -> Result<ResetPasswordResult, String> {
    match result {
        Ok(ok) => {
            if ok.status().is_success() {
                Ok(ResetPasswordResult::PasswordReset)
            } else if ok.status().is_client_error() {
                Ok(ResetPasswordResult::BadRequest(
                    decode_error_response(ok).await,
                ))
            } else {
                Err(format!("Unexpected server error '{}'", ok.status()))
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

pub async fn reset_password(input: ResetPasswordInput) -> Result<ResetPasswordResult, String> {
    let url = BASE_URL.to_owned() + RESET_PASSWORD_PATH;

    let body = match to_string(&input) {
        Ok(ok) => ok,
        Err(err) => return Err(err.to_string()),
    };

    println!("POST {url}");
    let client = reqwest::Client::new();
    let result = client
        .post(url)
        .body(body)
        .header(header::CONTENT_TYPE, "application/json")
        .send()
        .await;

    map_result(result).await
}
