use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData<T> {
    pub success: bool,
    pub message: String,
    pub error_code: Option<u32>,
    pub data: Option<T>,
}
