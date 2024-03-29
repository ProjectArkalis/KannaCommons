use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum KannaLists {
    MyAnimeList(i64),
    Anilist(i64),
}
