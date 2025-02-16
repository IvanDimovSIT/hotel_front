use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use crate::{
    app::GlobalState,
    constants::{BASE_URL, LOGOUT_PATH},
};

async fn logout_request(token: Option<String>) -> Result<(), Box<dyn Error>> {
    let url = BASE_URL.to_owned() + LOGOUT_PATH;
    let client = reqwest::Client::new();
    println!("POST {url}");
    let result = client
        .post(url)
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            format!("Bearer {}", token.unwrap_or_default()),
        )
        .send()
        .await?;

    if result.status().is_success() {
        Ok(())
    } else {
        let error_message = result.text().await?;
        Err(format!("Failed to logout: {}", error_message).into())
    }
}

pub async fn logout(global_state: Arc<Mutex<GlobalState>>) -> Result<(), String> {
    let token = {
        let guard = global_state.lock().unwrap();
        if let Some(some) = &guard.token {
            Some(some.token_string.clone())
        } else {
            None
        }
    };
    if token.is_none() {
        return Ok(());
    }
    let result = logout_request(token).await;
    match result {
        Ok(_) => Ok(()),
        Err(err) => {
            println!("{err}");
            Err("Failed to logout".to_owned())
        }
    }
}
