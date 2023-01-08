use serde::{Deserialize, Serialize};

use monie_rpc::monie::timeline::StoryResponse;

use crate::api::models::media::{AudioMedia, GraphicMedia};
use crate::api::models::user::User;
use crate::domain::models::story::StoryCore;

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct Story {
    pub id: String,
    pub author: User,
    pub created_at: u64,
    pub text: Option<String>,
    pub scene: GraphicMedia,
    pub soundtrack: Option<AudioMedia>,
}

impl From<Story> for StoryResponse {
    fn from(story: Story) -> Self {
        Self {
            id: story.id,
            text: story.text,
            created_at: story.created_at,
            author: Some(story.author.into()),
            scene: Some(story.scene.into()),
            soundtrack: story.soundtrack.map(|soundtrack| soundtrack.into()),
        }
    }
}

impl From<StoryCore> for Story {
    fn from(story: StoryCore) -> Self {
        Self {
            id: story.id,
            text: story.text,
            created_at: story.created_at,
            author: story.author.into(),
            scene: story.scene.into(),
            soundtrack: story.soundtrack.map(|soundtrack| soundtrack.into()),
        }
    }
}
