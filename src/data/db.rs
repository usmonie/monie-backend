use std::collections::HashMap;
use std::sync::Mutex;

use lazy_static::lazy_static;
use uuid::Uuid;

use crate::domain::models::media::GraphicMediaCore;
use crate::domain::models::session::{Session, UserSession};
use crate::domain::models::user::UserCore;

lazy_static! {
    static ref SESSIONS: Mutex<HashMap<Uuid, Session>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };

    static ref USERS_SESSIONS: Mutex<HashMap<Uuid, UserSession>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };

    static ref USERNAMES: Mutex<HashMap<String, Uuid>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}

pub const PASSWORD_PEPPER: &str = "gkHbhXQG3JIbvyGjI1GfsMAxSQgnI1XesBcfT7GcznBi7Htbd7MD0gJlmYlC5t";

pub fn create_tables() {
    // TODO: FIX THIS
}

pub fn is_session_id_exist(id: &Uuid) -> bool {
    SESSIONS.lock().unwrap().contains_key(id)
}

pub fn is_username_exist(username: &String) -> bool {
    USERNAMES.lock().unwrap().contains_key(username)
}

pub fn create_session(session_key: Vec<u8>) -> (Uuid, Option<Session>) {
    let mut uuid = Uuid::new_v4();
    let mut sessions = SESSIONS.lock().unwrap();
    while sessions.contains_key(&uuid) {
        uuid = Uuid::new_v4();
    }
    (uuid, sessions.insert(uuid, Session { session_key, user_id: None }))
}

pub fn get_session(uuid: &Uuid) -> Option<Session> {
    let sessions = SESSIONS.lock().unwrap();
    let session = sessions.get(uuid);

    session.cloned()
}

pub fn get_user_session(uuid: &Uuid) -> Option<UserSession> {
    let sessions = USERS_SESSIONS.lock().unwrap();
    let session = sessions.get(uuid);

    session.cloned()
}

pub fn get_username_id(username: &String) -> Option<Uuid> {
    let usernames = USERNAMES.lock().unwrap();
    let username = usernames.get(username);

    username.copied()
}

pub fn store_user(
    session_uuid: &Uuid,
    name: String,
    avatar: Option<GraphicMediaCore>,
    status: Option<String>,
    username: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    private_key: &Vec<u8>,
    password: [u8; 64],
    salt: &Vec<u8>,
) -> Option<UserSession> {
    let mut users = USERS_SESSIONS.lock().unwrap();
    let mut user_uuid = Uuid::new_v4();
    while users.contains_key(&user_uuid) {
        user_uuid = Uuid::new_v4();
    }

    let user = UserCore {
        id: user_uuid.to_string(),
        name,
        avatar,
        status,
        username,
        phone,
        email,
    };

    let user_session = UserSession {
        user,
        private_key: private_key.clone(),
        hashed_password: password.clone().to_vec(),
        salt: salt.clone(),
    };

    let mut sessions = SESSIONS.lock().unwrap();
    let session = sessions.get(session_uuid).unwrap();
    let new_session = Session { session_key: session.session_key.clone(), user_id: Some(user_uuid) };
    sessions.insert(*session_uuid, new_session);

    users.insert(user_uuid, user_session)
}

pub fn store_password(id: &Uuid, password: Vec<u8>, salt: Vec<u8>) {
    let mut sessions = USERS_SESSIONS.lock().unwrap();
    let user_session = sessions.get(id).unwrap();
    let new_user_session = UserSession {
        user: user_session.user.clone(),
        private_key: user_session.private_key.clone(),
        hashed_password: password,
        salt,
    };

    sessions.insert(*id, new_user_session);
}