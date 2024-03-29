use std::fmt::{self, Display, Formatter};
use bitflags::bitflags;

#[derive(PartialEq, Eq)]
pub struct Genre(u64);

bitflags! {
    impl Genre: u64 {
        const Unknown = 0;
        const Action = 1;
        const Comedy = 2;
        const Horror = 4;
        const Sports = 8;
        const Adventure = 16;
        const Drama = 32;
        const Mystery = 64;
        const Supernatural = 128;
        const AvantGarde = 256;
        const Fantasy = 512;
        const Romance = 1024;
        const Suspense = 2048;
        const AwardWinning = 4096;
        const GirlsLove = 8192;
        const SciFi = 16384;
        const BoysLove = 32768;
        const Gourmet = 65536;
        const SliceOfLife = 131072;
        const Ecchi = 262144;
        const Erotica = 524288;
        const Hentai = 1048576;
    }
}

//certamente tem outro jeito de fazer isso

impl Display for Genre {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Genre::Action => "Ação",
            Genre::Comedy => "Comédia",
            Genre::Horror => "Terror",
            Genre::Sports => "Esportes",
            Genre::Adventure => "Aventura",
            Genre::Drama => "Drama",
            Genre::Mystery => "Misterio",
            Genre::Supernatural => "Sobrenatural",
            Genre::AvantGarde => "Vanguarda",
            Genre::Fantasy => "Fantasia",
            Genre::Romance => "Romance",
            Genre::Suspense => "Suspense",
            Genre::AwardWinning => "Premiados",
            Genre::GirlsLove => "Amor entre garotas",
            Genre::SciFi => "Sci-Fi",
            Genre::BoysLove => "Amor entre garotos",
            Genre::Gourmet => "Gourmet",
            Genre::SliceOfLife => "Slice of Life",
            Genre::Ecchi => "Ecchi",
            Genre::Erotica => "Erotica",
            Genre::Hentai => "Hentai",
            Genre::Unknown => "Desconhecido",
            _ => "Genero Invalido",
        };

        write!(f, "{name}")
    }
}


impl From<String> for Genre {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Action" => Genre::Action,
            "Adventure" => Genre::Adventure,
            "Comedy" => Genre::Comedy,
            "Drama" => Genre::Drama,
            "Ecchi" => Genre::Ecchi,
            "Fantasy" => Genre::Fantasy,
            "Hentai" => Genre::Hentai,
            "Horror" => Genre::Horror,
            "Mahou Shoujo" => Genre::Unknown,
            "Mecha" => Genre::Unknown,
            "Music" => Genre::Unknown,
            "Mystery" => Genre::Mystery,
            "Psychological" => Genre::Unknown,
            "Romance" => Genre::Romance,
            "Sci-Fi" => Genre::SciFi,
            "Slice of Life" => Genre::SliceOfLife,
            "Sports" => Genre::Sports,
            "Supernatural" => Genre::Supernatural,
            "Thriller" => Genre::Suspense,
            _ => Genre::Unknown,
        }
    }
}