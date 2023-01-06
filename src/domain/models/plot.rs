use serde::Serialize;

use crate::domain::models::media::AudioMediaCore;
use crate::domain::models::story::StoryCore;
use crate::domain::models::user::UserCore;

#[derive(Serialize)]
pub struct PlotCore {
    pub id: String,
    pub author: UserCore,
    pub title: Option<String>,
    pub description: Option<String>,
    pub soundtrack: Option<AudioMediaCore>,
    pub stories: Vec<StoryCore>,
}