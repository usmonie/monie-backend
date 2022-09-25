use serde::{Deserialize, Serialize};
use crate::data::db::models::media::GraphicMedia;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub id: String,
    pub name: String,
    pub username: String,
    pub avatar: Option<Box<GraphicMedia>>,
}