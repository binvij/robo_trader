
use serde::Deserialize;
#[derive(Deserialize, Clone)]
pub struct OptionsContract {
    pub ticker: String,
    pub base_contract_code: String,
    pub expire_date: String,
    pub strike_price: String,
    pub option_type: String,
}
