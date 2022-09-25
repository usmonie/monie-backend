use serde::{Serialize, Deserialize};
use crate::domain::models::author::AuthorCore;
use crate::domain::models::music::MusicCore;
use crate::domain::models::media::MediaTypeCore;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Story {
    pub id: String,
    pub author: Box<AuthorCore>,
    pub created_at: u64,
    pub text: Option<String>,
    pub scene: Box<MediaTypeCore>,
    pub soundtrack: MusicCore
}