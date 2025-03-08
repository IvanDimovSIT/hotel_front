use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use iced_aw::date_picker::Date;
use reqwest::{Response, StatusCode};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    app::GlobalState,
    constants::{BASE_URL, FIND_UNOCCUPIED_ROOMS_PATH},
    utils::decode_error_response,
};

#[derive(Debug, Clone)]
pub struct FindUnoccupiedRoomsInput {
    pub start_date: Date,
    pub end_date: Date,
    pub minimum_capacity: Option<i16>,
    pub maximum_capacity: Option<i16>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FindUnoccupiedRoomsOutput {
    room_ids: Vec<Uuid>,
}

pub enum FindUnoccupiedRoomsResult {
    Found(Vec<Uuid>),
    BadRequest(String),
    Forbidden,
}

async fn map_response(
    response: Response,
) -> Result<FindUnoccupiedRoomsResult, Box<dyn Error + Send + Sync>> {
    Ok(if response.status().is_success() {
        let output: FindUnoccupiedRoomsOutput = response.json().await?;
        println!("Found results '{}'", output.room_ids.len());
        FindUnoccupiedRoomsResult::Found(output.room_ids)
    } else if response.status() == StatusCode::FORBIDDEN
        || response.status() == StatusCode::UNAUTHORIZED
    {
        FindUnoccupiedRoomsResult::Forbidden
    } else {
        FindUnoccupiedRoomsResult::BadRequest(decode_error_response(response).await)
    })
}

fn convert_input(input: FindUnoccupiedRoomsInput) -> Vec<(&'static str, String)> {
    let mut query_params = vec![
        ("startDate", input.start_date.to_string()),
        ("endDate", input.end_date.to_string()),
    ];

    input
        .minimum_capacity
        .map(|min| query_params.push(("minimumCapacity", format!("{min}"))));

    input
        .maximum_capacity
        .map(|max| query_params.push(("maximumCapacity", format!("{max}"))));

    query_params
}

async fn find_unoccupied_rooms_request(
    token: String,
    input: FindUnoccupiedRoomsInput,
) -> Result<FindUnoccupiedRoomsResult, Box<dyn Error + Send + Sync>> {
    let url = BASE_URL.to_owned() + FIND_UNOCCUPIED_ROOMS_PATH;
    let client = reqwest::Client::new();
    println!("GET {url}");
    let query_params = convert_input(input);

    let result = client
        .get(url)
        .query(&query_params)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    println!("Response:{result:?}");

    map_response(result).await
}

pub async fn find_unoccupied_rooms(
    global_state: Arc<Mutex<GlobalState>>,
    input: FindUnoccupiedRoomsInput,
) -> Result<FindUnoccupiedRoomsResult, String> {
    let token = if let Some(some_token) = global_state
        .lock()
        .unwrap()
        .token
        .as_ref()
        .map(|x| x.token_string.clone())
    {
        some_token
    } else {
        return Ok(FindUnoccupiedRoomsResult::Forbidden);
    };

    match find_unoccupied_rooms_request(token, input).await {
        Ok(ok) => Ok(ok),
        Err(err) => Err(err.to_string()),
    }
}
