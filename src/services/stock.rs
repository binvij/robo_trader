use std::collections::HashMap;

use chrono::{Datelike, Utc};

use crate::{
    helper,
    model::{
        self,
        http::{params::Details, resp::OptionsContract},
    },
};

pub async fn get_options_data_by_asset_name(
    asset_name: String,
) -> Result<Vec<OptionsContract>, Box<dyn std::error::Error>> {
    log::info!("request for options data on asset name **{}**", asset_name);
    let options_params = model::http::params::GetOptionsParams {
        cmd: helper::api_constants::CMD_GET_OPTIONS.to_string(),
        params: Details {
            ltr: String::from("FIX"),
            base_contract_code: asset_name,
        },
    };
    let mut query = HashMap::new();
    query.insert("q", options_params);

    let http_client = reqwest::Client::new();
    let resp = http_client
        .get(String::from(helper::api_constants::API_BASE_URL))
        .json(&query)
        .send()
        .await?;
    log::info!("response recvd. Parsing it...");
    let body = resp.text().await?;
    let options_data: Vec<OptionsContract> = serde_json::from_str(&body)?;
    log::info!("total count of options data:{}", options_data.len());

    let filtered_data = options_data.iter().filter(|d| {
        let orig_date = &d.expire_date; 
        let exp_date =   chrono::NaiveDate::parse_from_str(&orig_date, "%Y-%m-%d").unwrap();
        exp_date.month() == Utc::now().month()
    });

    let mut options: Vec<OptionsContract> = Vec::new();
    for (_, data) in filtered_data.enumerate() {
        options.push(data.clone());
    }

    Ok(options)
}
