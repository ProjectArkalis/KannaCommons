use super::source::KannaSource;
use crate::{
    aoba::Aoba,
    arkalis::{arkalis_api::GetSourcesBySeasonIdRequest, Arkalis},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KannaSeason {
    pub id: Option<u32>,
    pub name: String,
    pub thumbnail: Option<String>,
    pub sources: Vec<KannaSource>,
    pub sequence: u32,
}

impl KannaSeason {
    pub async fn save_thumb(&mut self, aoba: &Aoba) -> anyhow::Result<()> {
        if let Some(thumb) = &self.thumbnail {
            self.thumbnail = Some(aoba.upload(thumb).await?);
        }

        Ok(())
    }

    pub async fn with_sources(&mut self, arkalis: &mut Arkalis) -> anyhow::Result<&mut Self> {
        let sources = arkalis
            .client
            .get_sources_by_season_id(GetSourcesBySeasonIdRequest {
                season_id: self.id.unwrap(),
            })
            .await?
            .into_inner()
            .sources;

        self.sources = sources.into_iter().map(KannaSource::from).collect();
        for source in self.sources.iter_mut() {
            source.with_episodes(self.id.unwrap(), arkalis).await?;
        }

        Ok(self)
    }
}
