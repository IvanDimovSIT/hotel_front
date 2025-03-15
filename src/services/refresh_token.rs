use reqwest::header;
use serde::Deserialize;

use crate::{
    constants::{BASE_URL, REFRESH_TOKEN_PATH},
    security::JwtToken,
    utils::decode_error_response,
};

#[derive(Debug, Deserialize)]
struct RefreshTokenOutput {
    pub token: String,
}

pub async fn refresh_token(token: String) -> Result<JwtToken, String> {
    let url = BASE_URL.to_owned() + REFRESH_TOKEN_PATH;
    let client = reqwest::Client::new();
    println!("GET {url}");
    let result = client
        .get(url)
        .header(header::CONTENT_TYPE, "application/json")
        .bearer_auth(token)
        .send()
        .await;

    let result = match result {
        Ok(ok) => ok,
        Err(err) => return Err(format!("{err}")),
    };

    if result.status().is_success() {
        let refresh_token_output: Result<RefreshTokenOutput, _> = result.json().await;
        let refresh_token_output = match refresh_token_output {
            Ok(ok) => ok,
            Err(err) => return Err(format!("{err}")),
        };
        match JwtToken::new(refresh_token_output.token) {
            Some(jwt) => Ok(jwt),
            None => Err("Error constructing jwt".to_owned()),
        }
    } else {
        Err(decode_error_response(result).await)
    }
}
