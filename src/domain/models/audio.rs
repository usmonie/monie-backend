use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AudioCore {
    pub id: String,
    pub url: String,
    pub name: String,
    pub size: u64,
    pub created_at: u64,
    pub uploaded_at: u64,
    pub repeatable: bool,
    pub duration: u64,
}