use diesel::Queryable;
use serde::{Serialize, Deserialize};
use crate::data::db::models::author::Author;
use crate::data::db::models::music::Music;
use crate::data::db::models::story::Story;


#[derive(Serialize, Deserialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct Plot {
    pub id: String,
    pub author: Box<Author>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub soundtrack: Option<Box<Music>>,
    pub stories: Box<Vec<Story>>
}