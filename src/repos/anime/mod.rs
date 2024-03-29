use self::{lists::KannaLists, title::KannaTitle};
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
    
}
