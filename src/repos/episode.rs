use serde::{Deserialize, Serialize};

use crate::arkalis::{arkalis_api::CreateEpisodeRequest, Arkalis};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct KannaEpisode {
    pub id: Option<String>,
    pub thumbnail: Option<String>,
    pub is_hidden: bool,
    pub is_nsfw: bool,
    pub lbry_url: String,
    pub title: Option<String>,
    pub name: Option<String>,
}


impl KannaEpisode {
    pub async fn create_episode(
        &mut self,
        season_id: u32,
        source_id: u32,
        sequence: u32,
        arkalis: &mut Arkalis,
    ) -> anyhow::Result<&mut Self> {
        let ep = arkalis
            .client
            .create_episode(CreateEpisodeRequest {
                is_hidden: self.is_hidden,
                is_nsfw: self.is_nsfw,
                cover_id: None,
                season_id,
                sequence,
                source_id,
            })
            .await?
            .into_inner();
        self.id = Some(ep.id);
        self.name = Some(ep.name);
        Ok(self)
    }
}
