use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "enum", derive(clap::ValueEnum))]
pub enum SourceType {
    Raw = 1,
    EnglishDub = 2,
    PortugueseDub = 4,
    EnglishSub = 8,
    PortugueseSub = 16,
    NarcoSub = 32,
    NarcoDub = 64,
}

// pub struct SourceType(u64);

// bitflags! {
//     impl SourceType: u64 {
//         const Raw = 1;
//         const EnglishDub = 2;
//         const PortugueseDub = 4;
//         const EnglishSub = 8;
//         const PortugueseSub = 16;
//         const NarcoSub = 32;
//         const NarcoDub = 64;
//     }
// }

// impl Display for SourceType {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let name = match *self {
//             SourceType::Raw => "Raw",
//             SourceType::EnglishDub => "Dublagem em Inglês",
//             SourceType::PortugueseDub => "Dublagem em Português",
//             SourceType::EnglishSub => "Legendas em Inglês",
//             SourceType::PortugueseSub => "Legendas em Português",
//             SourceType::NarcoSub => "Legendas em Espanhol",
//             SourceType::NarcoDub => "Dublagem em Espanhol",
//             _ => "Unknown",
//         };

//         write!(f, "{name}")
//     }
// }
