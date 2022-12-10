use serde::{Deserialize, Serialize};

use crate::domain::models::author::AuthorCore;
use crate::domain::models::media::MediaTypeCore;
use crate::domain::models::music::MusicCore;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoryCore {
    pub id: String,
    pub author: Box<AuthorCore>,
    pub created_at: u64,
    pub text: Option<String>,
    pub scene: Box<MediaTypeCore>,
    pub soundtrack: MusicCore,
}