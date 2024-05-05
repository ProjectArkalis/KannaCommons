use serde::{Deserialize, Serialize};

use crate::arkalis::{
    arkalis_api::{
        CreateEpisodeRequest, GetEpisodeByIdRequest, GetEpisodesBySeasonAndSourceRequest,
        UpdateEpisodeRequest,
    },
    Arkalis,
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct KannaEpisode {
    pub id: Option<String>,
    pub thumbnail: Option<String>,
    pub is_hidden: bool,
    pub is_nsfw: bool,
    pub lbry_id: Option<String>,
    pub file_url: String,
    pub lbry_url: Option<String>,
    pub name: Option<String>,
}

impl KannaEpisode {
    pub async fn get_episodes(
        season: u32,
        source: u32,
        arkalis: &mut Arkalis,
    ) -> anyhow::Result<Vec<KannaEpisode>> {
        let eps = arkalis
            .client
            .get_episodes_by_season_and_source(GetEpisodesBySeasonAndSourceRequest {
                season_id: season,
                source_id: source,
            })
            .await?
            .into_inner();
        Ok(eps.episodes.into_iter().map(KannaEpisode::from).collect())
    }

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
        lbry_url: Option<String>,
        sequence: u32,
        arkalis: &mut Arkalis,
    ) -> anyhow::Result<&mut Self> {
        arkalis
            .client
            .update_episode(UpdateEpisodeRequest {
                id: id.clone(),
                lbry_url,
                is_hidden: self.is_hidden,
                cover_id: None,
                sequence,
            })
            .await?;

        let ep = arkalis
            .client
            .get_episode_by_id(GetEpisodeByIdRequest { id })
            .await?
            .into_inner()
            .episode
            .unwrap();
        self.lbry_id = Some(ep.lbry_media_id);
        self.file_url = ep.file_name;
        Ok(self)
    }
}
