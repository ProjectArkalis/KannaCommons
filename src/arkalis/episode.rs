use crate::repos::episode::KannaEpisode;

use super::arkalis_api::Episode;

impl From<Episode> for KannaEpisode {
    fn from(value: Episode) -> Self {
        KannaEpisode {
            id: Some(value.id),
            is_hidden: false,
            is_nsfw: value.is_nsfw,
            file_url: value.file_name,
            lbry_id: Some(value.lbry_media_id),
            name: Some(value.name),
            lbry_url: None,
            thumbnail: value.cover_id,
        }
    }
}
