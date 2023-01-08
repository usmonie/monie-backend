use serde::Serialize;

use monie_rpc::monie::timeline::{PlotResponse, StoryResponse};

use crate::api::models::media::AudioMedia;
use crate::api::models::story::Story;
use crate::api::models::user::User;
use crate::domain::models::plot::PlotCore;

#[derive(Serialize)]
pub struct Plot {
    pub id: String,
    pub author: User,
    pub title: Option<String>,
    pub description: Option<String>,
    pub soundtrack: Option<AudioMedia>,
    pub stories: Vec<Story>,
}

impl From<Plot> for PlotResponse {
    fn from(plot: Plot) -> Self {
        Self {
            id: plot.id,
            title: plot.title,
            description: plot.description,
            owner: Some(plot.author.into()),
            image: None,
            stories: plot.stories.iter().map(|s| StoryResponse::from(s.clone())).collect(),
        }
    }
}

impl From<PlotCore> for Plot {
    fn from(plot: PlotCore) -> Self {
        Self {
            id: plot.id,
            author: plot.author.into(),
            title: plot.title,
            description: plot.description,
            soundtrack: plot.soundtrack.map(|soundtrack| soundtrack.into()),
            stories: plot.stories.iter().map(|s| Story::from(s.clone())).collect(),
        }
    }
}