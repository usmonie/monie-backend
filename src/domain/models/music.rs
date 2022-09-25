use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct MusicCore {
    pub id: String,
    pub url: String,
    pub name: String,
    pub size: u64,
    pub created_at: u64,
    pub uploaded_at: u64,
    pub repeatable: bool,
    pub duration: u64
}

