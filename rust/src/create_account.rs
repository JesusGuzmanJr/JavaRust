use serde::Deserialize;
use validator::Validate;

#[derive(Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAccount {
    #[validate(length(min = 8, max = 16, message = "length must be 8 to 16 bytes"))]
    #[serde(default)]
    pub username: String,

    #[validate(email(message = "malformed"))]
    #[validate(length(max = 64, message = "length must be 64 bytes or less"))]
    #[serde(default)]
    pub email: String,
}
