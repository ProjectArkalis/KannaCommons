use kanna_models::user::User;

pub trait UserTraits {
    async fn create_user(name: String, adm_key: Option<String>) -> anyhow::Result<User>;
}

impl UserTraits for User {
    async fn create_user(name: String, adm_key: Option<String>) -> anyhow::Result<User> {
        todo!()
    }
}