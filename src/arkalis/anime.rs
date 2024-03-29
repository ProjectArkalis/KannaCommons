use crate::repos::anime::KannaAnime;

use super::arkalis_api::{AnimeInAnimeList, CreateAnimeRequest, Title};

impl From<KannaAnime> for CreateAnimeRequest {
    fn from(value: KannaAnime) -> Self {
        Self {
            titles: value.titles.into_iter().map(Title::from).collect(),
            synopsis: value.synopsis,
            thumbnail_id: None,
            banner_id: None,
            is_hidden: value.is_hidden,
            is_nsfw: value.is_nsfw,
            genre: value.genre,
            release_date: value.release_date,
            anime_in_lists: value
                .anime_lists
                .into_iter()
                .map(AnimeInAnimeList::from)
                .collect(),
        }
    }
}
