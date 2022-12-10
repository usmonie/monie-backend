use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ImageCore {
    pub id: String,
    pub url: String,
    pub name: String,
    pub size: u64,
    pub created_at: u128,
    pub uploaded_at: u128,
    pub height: u32,
    pub width: u32,
}