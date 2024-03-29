use crate::arkalis::{arkalis_api::CreateAnimeRequest, Arkalis};

use super::{anime::KannaAnime, season::KannaSeason};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnimeInfos {
    pub arkalis_id: Option<u32>,
    pub anime: KannaAnime,
    pub seassons: Vec<KannaSeason>,
}

impl AnimeInfos {
    pub async fn save(&mut self, conn: &mut Arkalis) -> anyhow::Result<&mut Self> {
        let resp = conn
            .create_anime(CreateAnimeRequest::from(self.anime.to_owned()))
            .await?
            .into_inner();
        self.arkalis_id = Some(resp.id);

        Ok(self)
    }
}
