use uuid::Uuid;

use crate::domain::models::user::UserCore;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct Session {
    pub session_key: Vec<u8>,
    pub user_id: Option<Uuid>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct UserSession {
    pub user: UserCore,
    pub private_key: Vec<u8>,
    pub hashed_password: Vec<u8>,
    pub salt: Vec<u8>,
}
