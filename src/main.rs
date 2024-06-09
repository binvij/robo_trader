mod helper;
mod model;
mod services;

use dotenv::dotenv;
use log;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new().init().unwrap();
    dotenv().ok();

    log::info!("program start");
    log::info!("acquiring user SID via authentication");
    let auth_token: model::user_auth_token::AuthToken;

    match services::auth::get_auth_token().await {
        Ok(val) => {
            auth_token = val;
        }
        Err(e) => {
            log::error!("error acquiring token.{}.", e);
            panic!("program terminating");
        }
    }

    log::info!("acquired token, {:?}", auth_token);

    let options = services::stock::get_options_data_by_asset_name(String::from("T.US")).await?;
    for data in options {
        log::info!(
            "Expire Data={}:Strike Price={}",
            data.expire_date,
            data.strike_price
        );
    }

    Ok(())
}
