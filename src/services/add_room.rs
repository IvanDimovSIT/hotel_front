use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use reqwest::{header, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use uuid::Uuid;

use crate::{
    app::GlobalState,
    components::validator::Validator,
    constants::{ADD_ROOM_PATH, BASE_URL},
    model::{bathroom_type::BathroomType, bed::Bed},
    utils::decode_error_response,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddRoomInput {
    pub beds: Vec<Bed>,
    pub price: i64,
    pub floor: i16,
    pub room_number: String,
    pub bathroom_type: BathroomType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AddRoomOutput {
    room_id: Uuid,
}

#[derive(Debug)]
pub enum AddRoomResult {
    Added(Uuid),
    Forbidden,
    BadRequest(String),
}

async fn add_room_request(
    input: AddRoomInput,
    token: Option<String>,
) -> Result<AddRoomResult, Box<dyn Error + Send + Sync>> {
    let token_string = if let Some(some) = token {
        some
    } else {
        return Ok(AddRoomResult::Forbidden);
    };

    let url = BASE_URL.to_owned() + ADD_ROOM_PATH;
    let client = reqwest::Client::new();
    println!("POST {url}");
    let result = client
        .post(url)
        .header(header::CONTENT_TYPE, "application/json")
        .bearer_auth(token_string)
        .body(to_string(&input)?)
        .send()
        .await?;

    if result.status().is_success() {
        let add_room_output: AddRoomOutput = result.json().await?;
        Ok(AddRoomResult::Added(add_room_output.room_id))
    } else if result.status() == StatusCode::FORBIDDEN
        || result.status() == StatusCode::UNAUTHORIZED
    {
        Ok(AddRoomResult::Forbidden)
    } else {
        Ok(AddRoomResult::BadRequest(
            decode_error_response(result).await,
        ))
    }
}

fn validate_beds(input: &AddRoomInput) -> Result<(), String> {
    for bed in &input.beds {
        Validator::validate_bed(bed)?
    }

    Ok(())
}

pub async fn add_room(
    global_state: Arc<Mutex<GlobalState>>,
    add_room_input: AddRoomInput,
) -> Result<AddRoomResult, String> {
    validate_beds(&add_room_input)?;
    Validator::validate_floor(add_room_input.floor)?;
    Validator::validate_price(add_room_input.price)?;
    let token = {
        let guard = global_state.lock().unwrap();
        guard.token.as_ref().map(|some| some.token_string.clone())
    };
    let add_room_result = add_room_request(add_room_input, token).await;

    match add_room_result {
        Ok(ok) => {
            println!("add room response {ok:?}");
            Ok(ok)
        }
        Err(err) => {
            println!("{err}");
            Err("Unexpected error".to_owned())
        }
    }
}
