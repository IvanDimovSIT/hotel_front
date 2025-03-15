use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use reqwest::{header, StatusCode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    app::GlobalState,
    constants::{BASE_URL, FIND_GUEST_PATH},
    utils::decode_error_response,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FindGuestInput {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<String>,
    pub ucn: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FindGuestOutput {
    guest_ids: Vec<Uuid>,
}

pub enum FindGuestResult {
    Found(Vec<Uuid>),
    BadRequest(String),
    Forbidden,
}

fn convert_input(input: FindGuestInput) -> Vec<(&'static str, String)> {
    let mut query_params = vec![];

    input
        .first_name
        .map(|first_name| query_params.push(("firstName", first_name)));

    input
        .last_name
        .map(|last_name| query_params.push(("lastName", last_name)));

    input
        .date_of_birth
        .map(|date_of_birth| query_params.push(("dateOfBirth", date_of_birth)));

    input
        .phone_number
        .map(|phone_number| query_params.push(("phoneNumber", phone_number)));

    input.ucn.map(|ucn| query_params.push(("ucn", ucn)));

    query_params
}

async fn find_guest_request(
    token: String,
    input: FindGuestInput,
) -> Result<FindGuestResult, Box<dyn Error + Send + Sync>> {
    let url = BASE_URL.to_owned() + FIND_GUEST_PATH;
    println!("GET {url}");

    let client = reqwest::Client::new();
    println!("GET {url}");
    let query_params = convert_input(input);

    let result = client
        .get(url)
        .query(&query_params)
        .header(header::CONTENT_TYPE, "application/json")
        .bearer_auth(token)
        .send()
        .await?;

    let status = result.status();
    if status.is_success() {
        let output: FindGuestOutput = result.json().await?;
        Ok(FindGuestResult::Found(output.guest_ids))
    } else if status == StatusCode::FORBIDDEN || status == StatusCode::UNAUTHORIZED {
        Ok(FindGuestResult::Forbidden)
    } else {
        Ok(FindGuestResult::BadRequest(
            decode_error_response(result).await,
        ))
    }
}

pub async fn find_guest(
    global_state: Arc<Mutex<GlobalState>>,
    input: FindGuestInput,
) -> Result<FindGuestResult, String> {
    let token = if let Some(some) = global_state.lock().unwrap().token.as_ref() {
        some.token_string.clone()
    } else {
        return Ok(FindGuestResult::Forbidden);
    };

    match find_guest_request(token, input).await {
        Ok(ok) => Ok(ok),
        Err(err) => Err(err.to_string()),
    }
}
