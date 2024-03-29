use crate::repos::anime::title::{KannaTitle, KannaTitleTypes};

use super::arkalis_api::{Title, TitleType};

impl From<KannaTitle> for Title {
    fn from(value: KannaTitle) -> Self {
        Title {
            is_main: value.is_main,
            name: value.title,
            title_type: match value.title_type {
                KannaTitleTypes::Romaji => TitleType::Romaji.into(),
                KannaTitleTypes::English => TitleType::English.into(),
                KannaTitleTypes::Portuguese => TitleType::Portuguese.into(),
                KannaTitleTypes::Native => TitleType::Native.into(),
            },
        }
    }
}
