use serde::{Deserialize, Serialize};

use crate::aoba::Aoba;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KannaSeason {
    pub id: Option<u32>,
    pub name: String,
    pub thumbnail: Option<String>,
}

impl KannaSeason {
    pub async fn save_thumb(&mut self, aoba: &Aoba) -> anyhow::Result<()> {
        if let Some(thumb) = &self.thumbnail {
            self.thumbnail = Some(aoba.upload(thumb).await?);
        }

        Ok(())
    }
}
