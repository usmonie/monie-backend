use serde::{Serialize, Deserialize};
use crate::domain::models::media::MediaTypeCore;

#[derive(Serialize, Deserialize)]
pub struct AuthorCore {
    pub id: String,
    pub name: String,
    pub username: String,
    pub avatar: Option<Box<MediaTypeCore>>,
}