use self::arkalis_api::arkalis_core_service_client::ArkalisCoreServiceClient;
use tonic::{
    service::{interceptor::InterceptedService, Interceptor},
    transport::Channel,
    Request, Status,
};

pub mod anime;
pub mod title;
pub mod anime_lists;

pub mod arkalis_api {
    tonic::include_proto!("arkalis");
}

pub type Arkalis = ArkalisCoreServiceClient<InterceptedService<Channel, AuthInterceptor>>;

pub struct AuthInterceptor {
    token: Option<String>,
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        if let Some(token) = &self.token {
            request.metadata_mut().insert(
                "authorization",
                format!("Bearer {}", token).parse().unwrap(),
            );
        }
        Ok(request)
    }
}

pub async fn get_arkalis_client(
    arkalis_url: &str,
    token: Option<String>,
) -> anyhow::Result<Arkalis> {
    let channel = Channel::from_shared(arkalis_url.to_owned())?
        .connect()
        .await?;
    let client = ArkalisCoreServiceClient::with_interceptor(channel, AuthInterceptor { token });
    Ok(client)
}
