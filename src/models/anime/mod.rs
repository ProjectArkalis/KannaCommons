use self::{anime_lists::KannaAnimeList, title::KannaTitle};
use serde::{Deserialize, Serialize};

pub mod anime_infos;
pub mod anime_lists;
pub mod title;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KannaAnime {
    pub titles: Vec<KannaTitle>,
    pub synopsis: String,
    pub thumbnail: Option<String>,
    pub banner: Option<String>,
    pub is_hidden: bool,
    pub is_nsfw: bool,
    pub genre: u64,
    pub release_date: i64,
    pub anime_lists: Vec<KannaAnimeList>,

    //deve ter algum jeito melhor de fazer isso
    #[serde(skip)]
    pub banner_id: Option<String>,
    #[serde(skip)]
    pub thumbnail_id: Option<String>,
}

// impl From<GetMediaMedia> for KannaAnime {
//     fn from(value: GetMediaMedia) -> Self {
//         let title = value.title.unwrap();
//         let date = value.start_date.unwrap();

//         let naive_date = NaiveDate::from_ymd_opt(
//             date.year.unwrap() as i32,
//             date.month.unwrap() as u32,
//             date.day.unwrap() as u32,
//         )
//         .unwrap();
//         let datetime = naive_date.and_hms_opt(0, 0, 0).unwrap();

//         let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(datetime, Utc);

//         KannaAnime {
//             titles: vec![
//                 KannaTitle {
//                     name: title.english.unwrap(),
//                     is_main: false,
//                     title_type: KannaTitleType::English,
//                 },
//                 KannaTitle {
//                     name: title.romaji.unwrap(),
//                     is_main: true,
//                     title_type: KannaTitleType::Romaji,
//                 },
//                 KannaTitle {
//                     name: title.native.unwrap(),
//                     is_main: false,
//                     title_type: KannaTitleType::Native,
//                 },
//             ],
//             synopsis: value.description.unwrap(),
//             thumbnail: value.cover_image.unwrap().extra_large,
//             thumbnail_id: None,
//             banner: value.banner_image,
//             banner_id: None,
//             is_hidden: false,
//             is_nsfw: value.is_adult.unwrap(),
//             genre: value
//                 .genres
//                 .unwrap()
//                 .into_iter()
//                 .map(|x| Genre::from(x.unwrap()).bits())
//                 .sum(),
//             release_date: datetime.timestamp(),
//             anime_lists: vec![
//                 KannaAnimeList::Anilist(value.id),
//                 KannaAnimeList::MyAnimeList(value.id_mal.unwrap()),
//             ],
//         }
//     }
// }

// impl From<arkalis_api::Anime> for KannaAnime {
//     fn from(value: arkalis_api::Anime) -> Self {
//         KannaAnime {
//             titles: value
//                 .titles
//                 .iter()
//                 .map(|x| KannaTitle {
//                     name: x.name.clone(),
//                     is_main: x.is_main,
//                     title_type: match x.title_type() {
//                         arkalis_api::TitleType::Romaji => KannaTitleType::Romaji,
//                         arkalis_api::TitleType::English => KannaTitleType::English,
//                         arkalis_api::TitleType::Portuguese => KannaTitleType::Portuguese,
//                         arkalis_api::TitleType::Native => KannaTitleType::Native,
//                     },
//                 })
//                 .collect::<Vec<KannaTitle>>(),
//             synopsis: value.synopsis,

//             //todo
//             thumbnail: value.thumbnail_id.clone(),
//             banner: value.banner_id.clone(),

//             thumbnail_id: value.thumbnail_id,
//             banner_id: value.banner_id,

//             is_hidden: value.is_hidden,
//             is_nsfw: value.is_nsfw,
//             genre: value.genre,
//             release_date: value.release_date,
//             anime_lists: value
//                 .anime_in_lists
//                 .iter()
//                 .map(|x| match x.anime_list() {
//                     arkalis_api::AnimeList::MyAnimeList => {
//                         KannaAnimeList::MyAnimeList(x.id_in_list.parse::<i64>().unwrap())
//                     }
//                     arkalis_api::AnimeList::Anilist => {
//                         KannaAnimeList::Anilist(x.id_in_list.parse::<i64>().unwrap())
//                     }
//                 })
//                 .collect::<Vec<KannaAnimeList>>(),
//         }
//     }
// }

// impl KannaAnime {
//     pub async fn save_images(&mut self, aoba: &AobaService) -> anyhow::Result<()> {
//         self.banner_id = if let Some(banner) = &self.banner {
//             let id = aoba.upload(banner).await?;
//             self.banner = Some(aoba.format(&id));

//             Some(id)
//         } else {
//             None
//         };

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
