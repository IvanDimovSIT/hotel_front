use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use reqwest::StatusCode;
use uuid::Uuid;

use crate::{
    app::GlobalState,
    constants::{BASE_URL, GET_ROOM_PATH},
    model::room::Room,
    utils::decode_error_response,
};

pub enum GetRoomResult {
    Found(Room),
    BadRequest(String),
    Forbidden,
}

async fn get_room_request(
    token: &str,
    id: Uuid,
) -> Result<GetRoomResult, Box<dyn Error + Send + Sync>> {
    let url = BASE_URL.to_owned() + GET_ROOM_PATH + &id.to_string();
    let client = reqwest::Client::new();

    println!("GET {url}");
    let response = client
        .get(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    let status = response.status();
    if status.is_success() {
        let room: Room = response.json().await?;
        Ok(GetRoomResult::Found(room))
    } else if status == StatusCode::FORBIDDEN || status == StatusCode::UNAUTHORIZED {
        Ok(GetRoomResult::Forbidden)
    } else {
        Ok(GetRoomResult::BadRequest(
            decode_error_response(response).await,
        ))
    }
}

pub async fn get_room(token_string: String, id: Uuid) -> Result<GetRoomResult, String> {
    match get_room_request(&token_string, id).await {
        Ok(ok) => Ok(ok),
        Err(err) => Err(err.to_string()),
    }
}
