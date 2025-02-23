use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use iced_aw::date_picker::Date;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use uuid::Uuid;

use crate::{
    app::GlobalState,
    constants::{ADD_GUEST_PATH, BASE_URL},
    model::id_card::{IdCard, IdCardDto},
    utils::decode_error_response,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddGuestInput {
    pub first_name: String,
    pub last_name: String,
    #[serde(skip)]
    pub date_of_birth_model: Date,
    pub date_of_birth: String,
    pub phone_number: Option<String>,
    #[serde(skip)]
    id_card_model: Option<IdCard>,
    id_card: Option<IdCardDto>,
}
impl AddGuestInput {
    pub fn new(
        first_name: String,
        last_name: String,
        date_of_birth: Date,
        phone_number: Option<String>,
        id_card: Option<IdCard>,
    ) -> Self {
        Self {
            id_card_model: id_card.clone(),
            id_card: id_card.map(|x| x.into()),
            first_name,
            last_name,
            date_of_birth: date_of_birth.to_string(),
            date_of_birth_model: date_of_birth,
            phone_number,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddGuestOutput {
    pub guest_id: Uuid,
}

pub enum AddGuestResult {
    GuestAdded(Uuid),
    Forbidden,
    BadRequest(String),
}

async fn add_guest_request(
    input: AddGuestInput,
    token: Option<String>,
) -> Result<AddGuestResult, Box<dyn Error + Send + Sync>> {
    let token_string = if let Some(some) = token {
        some
    } else {
        return Ok(AddGuestResult::Forbidden);
    };

    let url = BASE_URL.to_owned() + ADD_GUEST_PATH;
    let client = reqwest::Client::new();
    println!("POST {url}");
    println!("Input:{input:?}");
    let result = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token_string))
        .body(to_string(&input)?)
        .send()
        .await?;

    println!("Response:{result:?}");

    if result.status().is_success() {
        let add_guest_output: AddGuestOutput = result.json().await?;
        println!("Added guest '{}'", add_guest_output.guest_id);
        Ok(AddGuestResult::GuestAdded(add_guest_output.guest_id))
    } else if result.status() == StatusCode::FORBIDDEN
        || result.status() == StatusCode::UNAUTHORIZED
    {
        Ok(AddGuestResult::Forbidden)
    } else {
        Ok(AddGuestResult::BadRequest(
            decode_error_response(result).await,
        ))
    }
}

pub async fn add_guest(
    global_state: Arc<Mutex<GlobalState>>,
    add_guest_input: AddGuestInput,
) -> Result<AddGuestResult, String> {
    let token = {
        let guard = global_state.lock().unwrap();
        if let Some(some) = &guard.token {
            Some(some.token_string.clone())
        } else {
            None
        }
    };

    match add_guest_request(add_guest_input, token).await {
        Ok(res) => Ok(res),
        Err(err) => {
            println!("{err}");
            Err("Unexpected error".to_owned())
        }
    }
}
