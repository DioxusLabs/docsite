use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveToGistReq {
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveToGistRes {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetGistRes {
    pub id: String,
    pub code: String,
}