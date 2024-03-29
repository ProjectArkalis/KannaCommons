use serde::{Deserialize, Serialize};

use crate::models::season::KannaSeason;

use super::KannaAnime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnimeInfos {
    pub arkalis_id: Option<u32>,
    pub anime: KannaAnime,
    pub seasons: Vec<KannaSeason>,
}
