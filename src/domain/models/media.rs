use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct GraphicMediaCore {
    pub id: String,
    pub url: String,
    pub name: String,
    pub uploaded_by_id: String,
    pub size: u64,
    pub created_at: u64,
    pub uploaded_at: u64,
    pub height: u32,
    pub width: u32,

    pub media_type: GraphicMediaTypeCore,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub enum GraphicMediaTypeCore {
    Video {
        repeatable: bool,
        duration: u64,
    },
    Image,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct AudioMediaCore {
    pub id: String,
    pub url: String,
    pub name: String,
    pub uploaded_by_id: String,
    pub size: u64,
    pub created_at: u64,
    pub uploaded_at: u64,
    pub duration: u64,

    pub media_type: AudioMediaTypeCore,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub enum AudioMediaTypeCore {
    Audio,
    Music,
}