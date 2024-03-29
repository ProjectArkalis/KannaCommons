use serde::{Deserialize, Serialize};

// use crate::aoba::AobaService;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KannaSeason {
    pub id: Option<u32>,
    pub name: String,
    pub thumbnail: Option<String>,

    #[serde(skip)]
    pub thumbnail_id: Option<String>
}

// impl KannaSeason {
//     pub async fn save_thumb(&mut self, aoba: &AobaService) -> anyhow::Result<()> {
//         self.thumbnail_id = if let Some(thumb) = &self.thumbnail {
//             let id = aoba.upload(thumb).await?;
//             self.thumbnail = Some(aoba.format(&id));

//             Some(id)
//         } else {
//             None
//         };

//         Ok(())
//     }
// }