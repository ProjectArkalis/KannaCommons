use self::{lists::KannaLists, title::KannaTitle};
use crate::aoba::Aoba;
use serde::{Deserialize, Serialize};

pub mod lists;
pub mod title;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KannaAnime {
    pub titles: Vec<KannaTitle>,
    pub synopsis: String,
    pub thumbnail: Option<String>,
    pub banner: Option<String>,
    pub is_hidden: bool,
    pub is_nsfw: bool,
    pub genre: u64,
    pub release_date: i64,
    pub anime_lists: Vec<KannaLists>,
}

impl KannaAnime {
    pub async fn save_images(&mut self, aoba: &Aoba) -> anyhow::Result<&mut Self> {
        if let Some(thumb) = &self.thumbnail {
            self.thumbnail = Some(aoba.upload(thumb).await?);
        }

        if let Some(banner) = &self.banner {
            self.banner = Some(aoba.upload(banner).await?);
        }

        Ok(self)
    }
}
