use crate::{
    aoba::get_id,
    repos::{
        anime::{
            lists::KannaLists,
            title::{KannaTitle, KannaTitleTypes},
            KannaAnime,
        },
        anime_infos::AnimeInfos,
    },
};

use super::arkalis_api::{
    Anime, AnimeInAnimeList, AnimeList, CreateAnimeRequest, EditAnimeRequest, Title, TitleType,
};

impl From<KannaAnime> for CreateAnimeRequest {
    fn from(value: KannaAnime) -> Self {
        Self {
            titles: value.titles.into_iter().map(Title::from).collect(),
            synopsis: value.synopsis,
            thumbnail_id: get_id(&value.thumbnail),
            banner_id: get_id(&value.banner),
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

impl From<AnimeInfos> for EditAnimeRequest {
    fn from(value: AnimeInfos) -> Self {
        EditAnimeRequest {
            id: value.arkalis_id.unwrap(),
            anime_in_lists: value
                .anime
                .anime_lists
                .into_iter()
                .map(|x| x.into())
                .collect(),
            genre: value.anime.genre,
            release_date: value.anime.release_date,
            synopsis: value.anime.synopsis,
            titles: value.anime.titles.into_iter().map(|x| x.into()).collect(),
            banner_id: get_id(&value.anime.banner),
            thumbnail_id: get_id(&value.anime.thumbnail),
        }
    }
}

impl From<Anime> for KannaAnime {
    fn from(value: Anime) -> Self {
        KannaAnime {
            titles: value
                .titles
                .iter()
                .map(|x| KannaTitle {
                    title: x.name.clone(),
                    is_main: x.is_main,
                    title_type: match x.title_type() {
                        TitleType::Romaji => KannaTitleTypes::Romaji,
                        TitleType::English => KannaTitleTypes::English,
                        TitleType::Portuguese => KannaTitleTypes::Portuguese,
                        TitleType::Native => KannaTitleTypes::Native,
                    },
                })
                .collect::<Vec<KannaTitle>>(),
            synopsis: value.synopsis,

            //todo
            thumbnail: value.thumbnail_id.clone(),
            banner: value.banner_id.clone(),

            is_hidden: value.is_hidden,
            is_nsfw: value.is_nsfw,
            genre: value.genre,
            release_date: value.release_date,
            anime_lists: value
                .anime_in_lists
                .iter()
                .map(|x| match x.anime_list() {
                    AnimeList::MyAnimeList => {
                        KannaLists::MyAnimeList(x.id_in_list.parse::<i64>().unwrap())
                    }
                    AnimeList::Anilist => KannaLists::Anilist(x.id_in_list.parse::<i64>().unwrap()),
                })
                .collect(),
        }
    }
}

impl From<Anime> for AnimeInfos {
    fn from(value: Anime) -> Self {
        AnimeInfos {
            arkalis_id: Some(value.id),
            anime: KannaAnime::from(value),
            seasons: vec![],
        }
    }
}
