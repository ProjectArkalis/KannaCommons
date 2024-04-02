use crate::repos::source::KannaSource;

use super::arkalis_api::Sources;

impl From<Sources> for KannaSource {
    fn from(value: Sources) -> Self {
        KannaSource {
            id: Some(value.id),
            name: value.name,
            priority: value.priority,
            //todo
            source_types: vec![],
            episodes: vec![],
        }
    }
}