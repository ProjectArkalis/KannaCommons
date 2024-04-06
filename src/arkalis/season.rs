use crate::repos::season::KannaSeason;

use super::arkalis_api::Season;

impl From<Season> for KannaSeason {
    fn from(value: Season) -> Self {
        KannaSeason {
            id: Some(value.id),
            name: value.name,
            thumbnail: value.cover_id,
            sequence: value.sequence,
            sources: vec![]
        }
    }
}