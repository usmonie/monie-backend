use uuid::Uuid;

use crate::domain::models::user::UserCore;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct SessionCore {
    pub session_key: Vec<u8>,
    pub user_id: Option<Uuid>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct UserSessionCore {
    pub user: UserCore,
    pub private_key: Vec<u8>,
    pub hashed_password: Vec<u8>,
    pub salt: Vec<u8>,
}
