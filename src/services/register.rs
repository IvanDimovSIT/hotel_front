use std::error::Error;

use reqwest::{header, Response};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use uuid::Uuid;

use crate::{
    constants::{BASE_URL, REGISTER_PATH},
    utils::decode_error_response,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterInput {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RegisterUserOutput {
    pub user_id: Uuid,
}

pub enum RegisterResult {
    Registered(Uuid),
    BadRequest(String),
}

async fn map_result(result: Result<Response, impl Error>) -> Result<RegisterResult, String> {
    match result {
        Ok(res) => {
            if res.status().is_success() {
                let decode_success_result: Result<RegisterUserOutput, _> = res.json().await;
                decode_success_result.map_or_else(
                    |err| Err(err.to_string()),
                    |success| Ok(RegisterResult::Registered(success.user_id)),
                )
            } else if res.status().is_client_error() {
                Ok(RegisterResult::BadRequest(decode_error_response(res).await))
            } else {
                Err(decode_error_response(res).await)
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

pub async fn register(register_input: RegisterInput) -> Result<RegisterResult, String> {
    let url = BASE_URL.to_owned() + REGISTER_PATH;
    let client = reqwest::Client::new();
    println!("POST {url}");

    let body = match to_string(&register_input) {
        Ok(ok) => ok,
        Err(err) => {
            return Err(err.to_string());
        }
    };

    let result = client
        .post(url)
        .body(body)
        .header(header::CONTENT_TYPE, "application/json")
        .send()
        .await;

    map_result(result).await
}
