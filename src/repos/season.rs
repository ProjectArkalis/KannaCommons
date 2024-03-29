use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KannaSeason {
    pub id: Option<u32>,
    pub name: String,
    pub thumbnail: Option<String>,
}