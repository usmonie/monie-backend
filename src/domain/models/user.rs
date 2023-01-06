use serde::{Deserialize, Serialize};
use crate::domain::models::media::GraphicMediaCore;

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct UserCore {
    pub id: String,
    pub name: String,
    pub avatar: Option<GraphicMediaCore>,
    pub status: Option<String>,
    pub username: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

impl UserCore {
    fn is_anonymous(&self) -> bool {
        self.phone.is_none() && self.email.is_none()
    }
}