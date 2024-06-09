use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAuthCredential {
    pub login: String,
    pub password: String,
    #[serde(rename="rememberMe")]
    pub remember_me: String,
}
