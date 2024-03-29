use crate::arkalis::{
    arkalis_api::{
        CreateAdminRequest, CreateRecoveryKeyRequest, CreateTokenRequest, GetUserInfoRequest,
        RecoveryUserRequest,
    },
    Arkalis,
};
use anyhow::anyhow;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(u8)]
pub enum Roles {
    Admin,
    Uploader,
    User,
}

impl From<Roles> for String {
    fn from(value: Roles) -> Self {
        match value {
            Roles::Admin => "admin".to_string(),
            Roles::Uploader => "uploader".to_string(),
            Roles::User => "user".to_string(),
        }
    }
}

impl<'a> From<&'a str> for Roles {
    fn from(value: &'a str) -> Self {
        match value {
            "admin" => Roles::Admin,
            "uploader" => Roles::Uploader,
            "user" => Roles::User,
            _ => Roles::User,
        }
    }
}

pub struct KannaUser {
    pub id: String,
    pub name: String,
    pub role: Roles,
    pub token: Option<String>,
}

impl KannaUser {
    pub async fn create_user(
        name: String,
        adm_key: Option<String>,
        arkalis: &mut Arkalis,
    ) -> anyhow::Result<Self> {
        let token = if let Some(key) = adm_key {
            arkalis
                .client
                .create_admin(CreateAdminRequest {
                    display_name: name,
                    admin_master_key: key,
                })
                .await?
                .into_inner()
                .token
        } else {
            arkalis
                .client
                .create_token(CreateTokenRequest { display_name: name })
                .await?
                .into_inner()
                .token
        };

        let mut arkalis = Arkalis::new(&arkalis.url, &Some(token.clone())).await?;

        let user = arkalis.client.get_user_info(GetUserInfoRequest {}).await?;
        let mut user = KannaUser::from(user.into_inner());
        user.token = Some(token);

        Ok(user)
    }

    pub async fn from_recovery_key(key: String, arkalis: &mut Arkalis) -> anyhow::Result<Self> {
        let token = arkalis
            .client
            .recovery_user(RecoveryUserRequest { recovery_key: key })
            .await?
            .into_inner()
            .token;

        //todo make this better
        let mut arkalis = Arkalis::new(&arkalis.url, &Some(token.clone())).await?;

        let user = arkalis.client.get_user_info(GetUserInfoRequest {}).await?;
        let mut user = KannaUser::from(user.into_inner());
        user.token = Some(token);

        Ok(user)
    }

    pub async fn get_recovery_key(&self, arkalis: &mut Arkalis) -> anyhow::Result<String> {
        if let Some(token) = &self.token {
            let mut arkalis = Arkalis::new(&arkalis.url, &Some(token.to_owned())).await?;

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
