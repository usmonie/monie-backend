use crate::domain::models::story::Story;

use serde::{Serialize, Deserialize};

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
