use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client,
};

pub mod upload;

pub struct AobaService {
    client: Client,
    url: String,
}

impl AobaService {
    pub fn start(aoba_url: String, token: Option<&str>) -> anyhow::Result<Self> {
        let mut headers = HeaderMap::new();

        if let Some(token) = token {
            let mut auth_value = HeaderValue::from_str(&format!("Bearer {}", token))?;
            auth_value.set_sensitive(true);
            headers.insert(header::AUTHORIZATION, auth_value);
        }

        let client = Client::builder().default_headers(headers).build()?;
        Ok(AobaService {
            client,
            url: aoba_url,
        })
    }

    pub fn format(&self, id: &String) -> String {
        format!("{}/images/{}", self.url, id)
    }
}
