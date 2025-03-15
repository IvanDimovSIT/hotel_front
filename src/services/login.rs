use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use reqwest::header;
use serde::{Deserialize, Serialize};
use serde_json::to_string;

use crate::{
    app::GlobalState,
    constants::{BASE_URL, LOGIN_PATH},
    security::JwtToken,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct LoginInput {
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LoginOutput {
    token: String,
}

async fn login_request(
    email: String,
    password: String,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let input = LoginInput { email, password };

    let url = BASE_URL.to_owned() + LOGIN_PATH;

    let client = reqwest::Client::new();
    println!("POST {url}");
    let result = client
        .post(url)
        .header(header::CONTENT_TYPE, "application/json")
        .body(to_string(&input)?)
        .send()
        .await?;

    if result.status().is_success() {
        let login_output: LoginOutput = result.json().await?;
        Ok(login_output.token)
    } else {
        let error_message = result.text().await?;
        Err(format!("Failed to login: {}", error_message).into())
    }
}

pub async fn login(
    global_state: Arc<Mutex<GlobalState>>,
    email: String,
    password: String,
) -> Result<JwtToken, String> {
    {
        let lock = &global_state.lock().unwrap();
        lock.validator.validate_email(&email)?;
        lock.validator.validate_password(&password)?;
    }

    let login_result = login_request(email, password).await;
    match login_result {
        Ok(ok) => {
            let token = JwtToken::new(ok);
            if let Some(some_token) = token {
                Ok(some_token)
            } else {
                println!("Error creating token object");
                Err("Unexpected error".to_owned())
            }
        }
        Err(err) => {
            println!("{err}");
            Err("Incorrect credentials".to_owned())
        }
    }
}
