use crate::{
    aoba::{get_id, Aoba},
    arkalis::{
        arkalis_api::{
            AddSeasonRequest, CreateAnimeRequest, EditAnimeRequest, EditSeasonRequest,
            GetAnimeByIdRequest, GetAnimeSeasonsRequest, SearchAnimeRequest,
        },
        Arkalis,
    },
};
use anyhow::anyhow;

use super::{anime::KannaAnime, season::KannaSeason};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnimeInfos {
    pub arkalis_id: Option<u32>,
    pub anime: KannaAnime,
    pub seasons: Vec<KannaSeason>,
}

impl AnimeInfos {
    pub async fn search(
        title: Option<String>,
        synopsis: Option<String>,
        is_nsfw: Option<bool>,
        genre: Option<u64>,
        start_release_date: Option<i64>,
        end_release_date: Option<i64>,
        arkalis: &mut Arkalis,
    ) -> anyhow::Result<Vec<Self>> {
        let search = arkalis
            .client
            .search_anime(SearchAnimeRequest {
                title,
                synopsis,
                is_nsfw,
                genre,
                start_release_date,
                end_release_date,
            })
            .await?
            .into_inner()
            .animes;
        Ok(search.into_iter().map(AnimeInfos::from).collect())
    }

    pub async fn with_seasons(&mut self, arkalis: &mut Arkalis) -> anyhow::Result<&mut Self> {
        let a = arkalis
            .client
            .get_anime_seasons(GetAnimeSeasonsRequest {
                anime_id: self.arkalis_id.unwrap(),
            })
            .await?
            .into_inner()
            .seasons;

        self.seasons = a.into_iter().map(KannaSeason::from).collect();
        for season in self.seasons.iter_mut() {
            season.with_sources(arkalis).await?;
        }

        Ok(self)
    }

    pub async fn from_anime_id(id: u32, arkalis: &mut Arkalis) -> anyhow::Result<Self> {
        let anime = arkalis
            .client
            .get_anime_by_id(GetAnimeByIdRequest { id })
            .await?
            .into_inner()
            .anime;
        if let Some(anime) = anime {
            let seasons = arkalis
                .client
                .get_anime_seasons(GetAnimeSeasonsRequest { anime_id: id })
                .await?
                .into_inner()
                .seasons;

            Ok(AnimeInfos {
                arkalis_id: Some(id),
                anime: KannaAnime::from(anime),
                seasons: seasons
                    .iter()
                    .map(|x| KannaSeason {
                        id: Some(x.id),
                        name: x.name.clone(),
                        thumbnail: x.cover_id.clone(),
                        sources: vec![],
                        sequence: x.sequence
                    })
                    .collect(),
            })
        } else {
            Err(anyhow!("Anime not found"))
        }
    }

    pub async fn save(&mut self, arkalis: &mut Arkalis, aoba: &Aoba) -> anyhow::Result<&mut Self> {
        self.anime.save_images(aoba).await?;

        if self.arkalis_id.is_some() {
            arkalis
                .client
                .edit_anime(EditAnimeRequest::from(self.to_owned()))
                .await?;

            for season in self.seasons.iter_mut().enumerate() {
                season.1.save_thumb(aoba).await?;

                arkalis
                    .client
                    .edit_season(EditSeasonRequest {
                        id: season.1.id.unwrap(),
                        name: season.1.name.clone(),
                        cover_id: get_id(&season.1.thumbnail),
                        sequence: season.0 as u32,
                    })
                    .await?;

                for source in season.1.sources.iter_mut() {
                    source.save_episodes(season.1.id.unwrap(), arkalis).await?;
                }
            }
        } else {
            let resp = arkalis
                .client
                .create_anime(CreateAnimeRequest::from(self.anime.to_owned()))
                .await?
                .into_inner();
            self.arkalis_id = Some(resp.id);

            for season in self.seasons.iter_mut().enumerate() {
                season.1.save_thumb(aoba).await?;

                let season_id = arkalis
                    .client
                    .add_season(AddSeasonRequest {
                        name: season.1.name.clone(),
                        cover_id: get_id(&season.1.thumbnail),
                        sequence: season.0 as u32,
                        anime_id: self.arkalis_id.unwrap(),
                    })
                    .await?
                    .into_inner()
                    .id;
                season.1.id = Some(season_id);

                for ep in season.1.sources.iter_mut() {
                    ep.save_episodes(season_id, arkalis).await?;
                }
            }
        }

        Ok(self)
    }
}
