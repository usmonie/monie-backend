
use serde::{Serialize, Deserialize};
use crate::data::db::models::story::Story;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BlabCore {
    Story(Story),
    Message(MessageCore)
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MessageCore {

}
