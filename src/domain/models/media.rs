use serde::{Deserialize, Serialize};

use crate::domain::models::audio::AudioCore;
use crate::domain::models::image::ImageCore;
use crate::domain::models::music::MusicCore;
use crate::domain::models::video::VideoCore;

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum MediaTypeCore {
    GraphicMedia(GraphicMediaCore),
    AudioMedia(AudioMediaCore),
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum GraphicMediaCore {
    Image(ImageCore),
    Video(VideoCore),
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum AudioMediaCore {
    Audio(AudioCore),
    Music(MusicCore),
}