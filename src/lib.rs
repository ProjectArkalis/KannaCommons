use anyhow::Error;
use aoba::Aoba;
use arkalis::Arkalis;

pub mod aoba;
pub mod arkalis;
pub mod repos;

#[derive(Debug)]
pub struct Clients {
    pub aoba: Result<Aoba, Error>,
    pub arkalis: Result<Arkalis, Error>,
}

impl Clients {
    pub async fn new_clients(
        arkalis_url: &str,
        aoba_url: &str,
        token: &Option<String>,
    ) -> Clients {
        let arkalis = Arkalis::new(arkalis_url, token).await;
        let aoba = Aoba::start(aoba_url, token);

        Clients { arkalis, aoba }
    }
}
