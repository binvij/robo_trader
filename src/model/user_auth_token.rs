use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthToken {
    pub success: bool,
    pub logged: bool,
    #[serde(rename = "SID")]
    pub sid: String,
    #[serde(rename = "userId")]
    pub user_id: i32,
    pub real: bool,
    pub account_type: String,
    pub account_title: String,
    pub account_type_description: Option<String>,
}
