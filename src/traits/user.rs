use crate::arkalis::{
    arkalis_api::{CreateAdminRequest, CreateTokenRequest, GetUserInfoRequest},
    Arkalis,
};
use async_trait::async_trait;
use kanna_models::user::User;

#[async_trait]
pub trait UserTraits<T> {
    async fn create_user(&mut self, name: String, adm_key: Option<String>) -> anyhow::Result<T>;
}

#[async_trait]
impl UserTraits<User> for Arkalis {
    async fn create_user(&mut self, name: String, adm_key: Option<String>) -> anyhow::Result<User> {
        let token = if let Some(key) = adm_key {
            self.client
                .create_admin(CreateAdminRequest {
                    admin_master_key: key,
                    display_name: name,
                })
                .await?
                .into_inner()
                .token
        } else {
            self.client
                .create_token(CreateTokenRequest { display_name: name })
                .await?
                .into_inner()
                .token
        };

        let mut arkalis  = Arkalis::new(&self.url, Some(token.as_str())).await?;
        let sexo = arkalis.client.get_user_info(GetUserInfoRequest {}).await?.into_inner();

        println!("{:?}", sexo);
        todo!()
    }
}
