use super::arkalis_api::GetUserInfoResponse;
use crate::repos::user::{KannaUser, Roles};

impl From<GetUserInfoResponse> for KannaUser {
    fn from(value: GetUserInfoResponse) -> Self {
        KannaUser {
            id: value.id,
            name: value.display_name,
            role: Roles::from(value.role.as_str()),
            token: None,
        }
    }
}
