use crate::arkalis::{
    arkalis_api::{
        CreateAdminRequest, CreateRecoveryKeyRequest, CreateTokenRequest, GetUserInfoRequest,
        RecoveryUserRequest,
    },
    Arkalis,
};
use anyhow::anyhow;
use async_trait::async_trait;
use kanna_models::user::{Roles, User};

//todo fazer isso ser um from ??
pub fn role_from_string(role: &str) -> Roles {
    match role {
        "admin" => Roles::Admin,
        "uploader" => Roles::Uploader,
        "user" => Roles::User,
        _ => Roles::User,
    }
}

#[async_trait]
pub trait UserTraits<T> {
    async fn create_user(&mut self, name: String, adm_key: Option<String>) -> anyhow::Result<User>;
    async fn from_recovery_key(&mut self, recovery_key: String) -> anyhow::Result<User>;
    async fn create_recovery_key(&mut self, user: User) -> anyhow::Result<String>;
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

        let mut arkalis = Arkalis::new(&self.url, Some(token.as_str())).await?;
        let user = arkalis
            .client
            .get_user_info(GetUserInfoRequest {})
            .await?
            .into_inner();

        Ok(User {
            id: user.id,
            name: user.display_name,
            token: Some(token),
            role: role_from_string(&user.role),
        })
    }

    async fn from_recovery_key(&mut self, recovery_key: String) -> anyhow::Result<User> {
        let token = self
            .client
            .recovery_user(RecoveryUserRequest { recovery_key })
            .await?
            .into_inner()
            .token;
        let mut arkalis = Arkalis::new(&self.url, Some(&token)).await?;
        let user = arkalis
            .client
            .get_user_info(GetUserInfoRequest {})
            .await?
            .into_inner();

        Ok(User {
            id: user.id,
            name: user.display_name,
            token: Some(token),
            role: role_from_string(&user.role),
        })
    }

    async fn create_recovery_key(&mut self, user: User) -> anyhow::Result<String> {
        if let Some(token) = &user.token {
            //su sinceramente n faço ideia pq isso existe, mas n vou mexer
            let mut arkalis = Arkalis::new(&self.url, Some(&token)).await?;

            Ok(arkalis
                .client
                .create_recovery_key(CreateRecoveryKeyRequest {})
                .await?
                .into_inner()
                .recovery_key)
        } else {
            Err(anyhow!("Invalid token"))
        }
    }
}
