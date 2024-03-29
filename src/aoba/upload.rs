use serde::{Deserialize, Serialize};

use super::Aoba;

#[derive(Serialize, Deserialize)]
struct Image {
    pub url: String,
}

impl Aoba {
    pub async fn upload(&self, img: &str) -> anyhow::Result<String> {
        let resp = self
            .client
            .post(format!("{}/images/url", self.url))
            .json(&Image {
                url: img.to_owned(),
            })
            .send()
            .await?
            .text()
            .await?;
        Ok(self.format(&resp))
    }
}
