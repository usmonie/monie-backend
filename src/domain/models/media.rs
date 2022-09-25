use serde::{Serialize, Deserialize};

use crate::domain::models::audio::AudioCore;
use crate::domain::models::gif::GifCore;
use crate::domain::models::music::MusicCore;
use crate::domain::models::video::VideoCore;
use crate::domain::models::image::ImageCore;


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MediaTypeCore {
    GraphicMedia(GraphicMediaCore),
    AudioMedia(AudioMediaCore)
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GraphicMediaCore {
    Image(ImageCore),
    Video(VideoCore),
    Gif(GifCore)
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AudioMediaCore {
    Audio(AudioCore),
    Music(MusicCore)
}