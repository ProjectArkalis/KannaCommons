use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum KannaAnimeList {
    MyAnimeList(i64),
    Anilist(i64),
}
