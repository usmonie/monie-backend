use serde::{Deserialize, Serialize};

use crate::domain::models::story::StoryCore;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BlabCore {
    Story(StoryCore),
    Message(MessageCore),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MessageCore {}
