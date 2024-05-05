use anyhow::Error;
use aoba::Aoba;
use arkalis::Arkalis;

pub mod aoba;
pub mod arkalis;
pub mod traits;

#[derive(Debug)]
pub struct Clients {
    pub aoba: Result<Aoba, Error>,
    pub arkalis: Result<Arkalis, Error>,
}

impl Clients {
    pub async fn new(arkalis_url: &str, aoba_url: &str, token: Option<&str>) -> Clients {
        let arkalis = Arkalis::new(arkalis_url, token).await;
        let aoba = Aoba::new(aoba_url, token);

        Clients { arkalis, aoba }
    }
}
