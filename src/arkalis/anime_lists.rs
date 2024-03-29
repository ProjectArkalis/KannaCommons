use super::arkalis_api::{AnimeInAnimeList, AnimeList};
use crate::repos::anime::lists::KannaLists;

impl From<KannaLists> for AnimeInAnimeList {
    fn from(value: KannaLists) -> Self {
        match value {
            KannaLists::Anilist(id) => AnimeInAnimeList {
                id_in_list: id.to_string(),
                anime_list: AnimeList::Anilist.into(),
            },
            KannaLists::MyAnimeList(id) => AnimeInAnimeList {
                id_in_list: id.to_string(),
                anime_list: AnimeList::MyAnimeList.into(),
            },
        }
    }
}
