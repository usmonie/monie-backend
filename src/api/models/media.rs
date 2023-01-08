use serde::{Deserialize, Serialize};

use monie_rpc::monie::media::{AudioResponse, GraphicResponse};

use crate::domain::models::media::{
    AudioMediaCore, AudioMediaTypeCore, GraphicMediaCore, GraphicMediaTypeCore,
};

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct GraphicMedia {
    pub id: String,
    pub url: String,
    pub name: String,
    pub uploaded_by_id: String,
    pub size: u64,
    pub created_at: u64,
    pub uploaded_at: u64,
    pub height: u32,
    pub width: u32,

    pub media_type: GraphicMediaType,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub enum GraphicMediaType {
    Video { repeatable: bool, duration: u64 },
    Image,
}

impl From<GraphicMedia> for GraphicResponse {
    fn from(graphic_media: GraphicMedia) -> Self {
        let (repeatable, duration) = match graphic_media.media_type {
            GraphicMediaType::Video {
                repeatable,
                duration,
            } => (repeatable, duration),
            GraphicMediaType::Image => (false, 0),
        };

        Self {
            id: graphic_media.id,
            uploaded_by_id: graphic_media.uploaded_by_id,
            name: graphic_media.name,
            size: graphic_media.size,
            created_at: graphic_media.created_at,
            upload_at: graphic_media.uploaded_at,
            height: graphic_media.height,
            width: graphic_media.width,
            repeatable,
            duration,
        }
    }
}

impl From<GraphicMediaCore> for GraphicMedia {
    fn from(graphic_media: GraphicMediaCore) -> Self {
        Self {
            id: graphic_media.id,
            url: graphic_media.url,
            name: graphic_media.name,
            uploaded_by_id: graphic_media.uploaded_by_id,
            size: graphic_media.size,
            created_at: graphic_media.created_at,
            uploaded_at: graphic_media.uploaded_at,
            height: graphic_media.height,
            width: graphic_media.width,
            media_type: graphic_media.media_type.into(),
        }
    }
}

impl From<GraphicMediaTypeCore> for GraphicMediaType {
    fn from(audio_media_type: GraphicMediaTypeCore) -> Self {
        match audio_media_type {
            GraphicMediaTypeCore::Image => GraphicMediaType::Image {},
            GraphicMediaTypeCore::Video {
                repeatable,
                duration,
            } => GraphicMediaType::Video {
                repeatable,
                duration,
            },
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct AudioMedia {
    pub id: String,
    pub url: String,
    pub name: String,
    pub uploaded_by_id: String,
    pub size: u64,
    pub created_at: u64,
    pub uploaded_at: u64,
    pub duration: u64,

    pub media_type: AudioMediaType,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub enum AudioMediaType {
    Audio,
    Music,
}

impl From<AudioMedia> for AudioResponse {
    fn from(audio_media: AudioMedia) -> Self {
        Self {
            id: audio_media.id,
            uploaded_by_id: audio_media.uploaded_by_id,
            name: audio_media.name,
            size: 0,
            created_at: 0,
            upload_at: 0,
            repeatable: false,
            duration: 0,
        }
    }
}

impl From<AudioMediaCore> for AudioMedia {
    fn from(audio_media: AudioMediaCore) -> Self {
        Self {
            id: audio_media.id,
            url: audio_media.url,
            name: audio_media.name,
            uploaded_by_id: audio_media.uploaded_by_id,
            size: audio_media.size,
            created_at: audio_media.created_at,
            uploaded_at: audio_media.uploaded_at,
            duration: audio_media.duration,
            media_type: audio_media.media_type.into(),
        }
    }
}

impl From<AudioMediaTypeCore> for AudioMediaType {
    fn from(audio_media_type: AudioMediaTypeCore) -> Self {
        match audio_media_type {
            AudioMediaTypeCore::Audio => AudioMediaType::Audio {},
            AudioMediaTypeCore::Music => AudioMediaType::Music {},
        }
    }
}
