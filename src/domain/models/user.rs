use crate::data::db::create_user;
use crate::domain::models::media::GraphicMediaCore;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct UserCore {
    pub id: String,
    pub name: String,
    pub avatar: Option<GraphicMediaCore>,
    pub about: Option<String>,
    pub username: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub created_at: u128,
}

impl UserCore {
    pub fn new(id: String, name: String) -> Self {
        UserCore {
            id,
            name,
            created_at: get_epoch_ms(),
            avatar: None,
            about: None,
            username: None,
            phone: None,
            email: None,
        }
    }
    pub fn new_with_username(id: String, name: String, username: Option<String>) -> Self {
        UserCore {
            id,
            name,
            created_at: get_epoch_ms(),
            avatar: None,
            about: None,
            username,
            phone: None,
            email: None,
        }
    }
    fn is_anonymous(&self) -> bool {
        self.phone.is_none() && self.email.is_none()
    }
}

fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
