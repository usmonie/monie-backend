use serde::{Deserialize, Serialize};

use crate::domain::models::media::{AudioMediaCore, GraphicMediaCore};
use crate::domain::models::user::UserCore;

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct StoryCore {
    pub id: String,
    pub author: UserCore,
    pub created_at: u64,
    pub text: Option<String>,
    pub scene: GraphicMediaCore,
    pub soundtrack: Option<AudioMediaCore>,
}