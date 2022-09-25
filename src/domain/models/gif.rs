
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GifCore {
    pub id: String,
    pub url: String,
    pub name: String,
    pub size: u64,
    pub created_at: u64,
    pub uploaded_at: u64,
    pub height: u32,
    pub width: u32,
    pub repeatable: bool,
    pub duration: u64
}