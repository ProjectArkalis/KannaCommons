use super::{episode::KannaEpisode, source_types::SourceType};
use crate::arkalis::{arkalis_api::CreateSourceRequest, Arkalis};
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
            episode
                .create_episode(season_id, self.id.unwrap(), i as u32, arkalis)
                .await?;
        }
        Ok(self)
    }
}
