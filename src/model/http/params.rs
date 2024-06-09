use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
pub struct Details {
    pub ltr: String,
    pub base_contract_code: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetOptionsParams {
    pub cmd: String,
    pub params: Details,
}
