use serde::Serialize;

use crate::domain::models::author::AuthorCore;
use crate::domain::models::music::MusicCore;
use crate::domain::models::story::StoryCore;

#[derive(Serialize)]
pub struct PlotCore {
    pub id: String,
    pub author: Box<AuthorCore>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub soundtrack: Option<Box<MusicCore>>,
    pub stories: Box<Vec<StoryCore>>,
}