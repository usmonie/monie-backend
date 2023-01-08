use serde::{Deserialize, Serialize};

use crate::api::models::media::GraphicMedia;
use monie_rpc::monie::user::UserResponse;

use crate::domain::models::user::UserCore;

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub avatar: Option<GraphicMedia>,
    pub about: Option<String>,
    pub username: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

impl User {
    fn is_anonymous(&self) -> bool {
        self.phone.is_none() && self.email.is_none()
    }
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            avatar: user.avatar.map(|avatar| avatar.into()),
            status: user.about,
            username: user.username,
            phone: user.phone,
            email: user.email,
        }
    }
}

impl From<UserCore> for User {
    fn from(user: UserCore) -> Self {
        Self {
            id: user.id,
            name: user.name,
            avatar: user.avatar.map(|avatar| avatar.into()),
            about: user.about,
            username: user.username,
            phone: user.phone,
            email: user.email,
        }
    }
}
