use aoba::Aoba;
use arkalis::Arkalis;

pub mod aoba;
pub mod arkalis;
pub mod repos;

pub async fn get_clients(
    arkalis_url: &str,
    aoba_url: &String,
    token: &Option<String>,
) -> anyhow::Result<(Arkalis, Aoba)> {
    let arkalis = Arkalis::new(arkalis_url, token).await?;
    let aoba = Aoba::start(aoba_url, token)?;

    Ok((arkalis, aoba))
}
