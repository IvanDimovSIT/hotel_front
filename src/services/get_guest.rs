use std::error::Error;

use reqwest::{header, StatusCode};
use uuid::Uuid;

use crate::{
    constants::{BASE_URL, GET_GUEST_PATH},
    model::guest::{Guest, GuestDto},
    utils::decode_error_response,
};

pub enum GetGuestResult {
    Found(Box<Guest>),
    BadRequest(String),
    Forbidden,
}

async fn get_guest_request(
    token: &str,
    id: Uuid,
) -> Result<GetGuestResult, Box<dyn Error + Send + Sync>> {
    let url = BASE_URL.to_owned() + GET_GUEST_PATH + &id.to_string();
    let client = reqwest::Client::new();

    println!("GET {url}");
    let response = client
        .get(url)
        .header(header::CONTENT_TYPE, "application/json")
        .bearer_auth(token)
        .send()
        .await?;

    let status = response.status();
    if status.is_success() {
        let guest_dto: GuestDto = response.json().await?;
        let guest = Box::new(guest_dto.convert_with_id(id));
        Ok(GetGuestResult::Found(guest))
    } else if status == StatusCode::FORBIDDEN || status == StatusCode::UNAUTHORIZED {
        Ok(GetGuestResult::Forbidden)
    } else {
        Ok(GetGuestResult::BadRequest(
            decode_error_response(response).await,
        ))
    }
}

pub async fn get_guest(token: String, guest_id: Uuid) -> Result<GetGuestResult, String> {
    match get_guest_request(&token, guest_id).await {
        Ok(ok) => Ok(ok),
        Err(err) => Err(err.to_string()),
    }
}
