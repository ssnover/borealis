use serde::{Deserialize};

#[derive(Deserialize)]
pub struct AddUserResponseBody {
    pub auth_token: String,
}