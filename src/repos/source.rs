use super::{episode::KannaEpisode, source_types::SourceType};
use crate::arkalis::{
    arkalis_api::{CreateSourceRequest, GetEpisodesBySeasonAndSourceRequest, GetSourceByIdRequest},
    Arkalis,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KannaSource {
    pub id: Option<u32>,
    pub name: String,
    pub priority: u32,
    pub source_types: Vec<SourceType>,
    pub episodes: Vec<KannaEpisode>,
}

impl KannaSource {
    pub async fn from_id(id: u32, arkalis: &mut Arkalis) -> anyhow::Result<Self> {
        let source = arkalis
            .client
            .get_source_by_id(GetSourceByIdRequest { id })
            .await?
            .into_inner()
            .source;

        if let Some(source) = source {
            Ok(source.into())
        } else {
            Err(anyhow::anyhow!("Source não encontrada"))
        }
    }

    pub async fn save(&mut self, arkalis: &mut Arkalis) -> anyhow::Result<&mut Self> {
        let id = arkalis
            .client
            .create_source(CreateSourceRequest {
                name: self.name.clone(),
                priority: self.priority,
                source_type: self
                    .source_types
                    .clone()
                    .into_iter()
                    .map(|x| x as u64)
                    .sum::<u64>(),
            })
            .await?
            .into_inner()
            .id;

        self.id = Some(id);

        Ok(self)
    }

    pub async fn save_episodes(
        &mut self,
        season_id: u32,
        arkalis: &mut Arkalis,
    ) -> anyhow::Result<&mut Self> {
        for (i, episode) in self.episodes.iter_mut().enumerate() {
            if let Some(id) = &episode.id {
                episode
                    .update_episode(id.to_owned(), episode.lbry_url.to_owned(), i as u32, arkalis)
                    .await?;
            } else {
                episode
                    .create_episode(season_id, self.id.unwrap(), i as u32, arkalis)
                    .await?;
            }
            episode.lbry_url = None;
        }
        Ok(self)
    }

    pub async fn with_episodes(
        &mut self,
        season_id: u32,
        arkalis: &mut Arkalis,
    ) -> anyhow::Result<&mut Self> {
        let eps = arkalis
            .client
            .get_episodes_by_season_and_source(GetEpisodesBySeasonAndSourceRequest {
                season_id,
                source_id: self.id.unwrap(),
            })
            .await?
            .into_inner()
            .episodes;

        self.episodes = eps.into_iter().map(KannaEpisode::from).collect();

        Ok(self)
    }
}
