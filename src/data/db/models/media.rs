use serde::{Serialize, Deserialize};
use crate::data::db::models::gif::Gif;
use crate::data::db::models::audio::Audio;
use crate::data::db::models::image::Image;
use crate::data::db::models::music::Music;
use crate::data::db::models::video::Video;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MediaType {
    GraphicMedia(GraphicMedia),
    AudioMedia(AudioMedia)
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GraphicMedia {
    Image(Image),
    Video(Video),
    Gif(Gif)
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AudioMedia {
    Audio(Audio),
    Music(Music)
}