use serde::{Deserialize, Serialize};

use crate::arkalis::{
    arkalis_api::{CreateEpisodeRequest, UpdateEpisodeRequest},
    Arkalis,
};

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

    pub async fn update_episode(
        &mut self,
        id: String,
        lbry_url: String,
        sequence: u32,
        arkalis: &mut Arkalis,
    ) -> anyhow::Result<&mut Self> {
        arkalis
            .client
            .update_episode(UpdateEpisodeRequest {
                id,
                lbry_url: Some(lbry_url),
                is_hidden: self.is_hidden,
                cover_id: None,
                sequence,
            })
            .await?;
        Ok(self)
    }
}
