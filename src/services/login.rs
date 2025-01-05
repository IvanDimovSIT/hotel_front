use std::error::Error;

use serde::{Deserialize, Serialize};
use serde_json::to_string;

use crate::constants::{BASE_URL, LOGIN_PATH};

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

pub async fn login(
    email: String,
    password: String,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let input = LoginInput { email, password };

    let url = BASE_URL.to_owned() + LOGIN_PATH;

    let client = reqwest::Client::new();
    let result = client
        .post(url)
        .header("Content-Type", "application/json")
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
