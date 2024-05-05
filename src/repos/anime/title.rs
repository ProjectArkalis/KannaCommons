use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum KannaTitleTypes {
    Romaji,
    English,
    Portuguese,
    Native,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KannaTitle {
    pub title: String,
    pub is_main: bool,
    pub title_type: KannaTitleTypes,
}

pub trait GetMain {
    fn get_main(&self) -> KannaTitle;
}

impl GetMain for Vec<KannaTitle> {
    fn get_main(&self) -> KannaTitle {
        self.iter().find(|x| x.is_main).unwrap().to_owned()
    }
}