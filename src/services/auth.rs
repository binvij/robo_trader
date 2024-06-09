use std::error::Error;

use crate::{helper::api_constants, model};

pub async fn get_auth_token() -> Result<model::user_auth_token::AuthToken, Box<dyn Error>> {
    log::info!("get_auth_token() called");
    let client = reqwest::Client::new();

    let user_cred = model::user_auth_cred::UserAuthCredential {
        login: std::env::var("TRADER_USER_LOGIN").expect("environment variable TRADER_USER_LOGIN not found"),
        password: std::env::var("TRADER_USER_PASSWORD").expect("environment variable TRADER_USER_PASSWORD not found"),
        remember_me: String::from("1"),
    };

    log::debug!("user cred={:#?}", user_cred);
    let post_url = format!("{}/check-login-password", api_constants::API_BASE_URL);
    log::info!("post url={}", post_url);
    let resp = client.post(post_url).form(&user_cred).send().await?;
    log::info!("auth api request made");
    let body = resp.text().await?;
    log::info!("auth response={}", body);

    let auth_token: model::user_auth_token::AuthToken = serde_json::from_str(&body)?;

    Ok(auth_token)
}
