use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client,
};

pub mod upload;

#[derive(Debug)]
pub struct Aoba {
    client: Client,
    url: String,
}

impl Aoba {
    pub fn new(aoba_url: &str, token: Option<&str>) -> anyhow::Result<Self> {
        let mut headers = HeaderMap::new();

        if let Some(token) = token {
            let mut auth_value = HeaderValue::from_str(&format!("Bearer {}", token))?;
            auth_value.set_sensitive(true);
            headers.insert(header::AUTHORIZATION, auth_value);
        }

        let client = Client::builder().default_headers(headers).build()?;
        Ok(Aoba {
            client,
            url: aoba_url.to_owned(),
        })
    }

    pub fn format(&self, id: &String) -> String {
        format!("{}/images/{}", self.url, id)
    }
}

pub(crate) fn get_id(url: &Option<String>) -> Option<String> {
    //realmente, mt complexo
    url.as_ref()
        .map(|url| url.split('/').last().unwrap().to_owned())
}
