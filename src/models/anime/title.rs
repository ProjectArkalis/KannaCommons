use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum KannaTitleType {
    Romaji,
    English,
    Portuguese,
    Native,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KannaTitle {
    pub name: String,
    pub is_main: bool,
    pub title_type: KannaTitleType,
}
