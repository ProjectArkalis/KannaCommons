use self::arkalis_api::arkalis_core_service_client::ArkalisCoreServiceClient;
use tonic::{
    service::{interceptor::InterceptedService, Interceptor},
    transport::{Channel, ClientTlsConfig},
    Request, Status,
};

pub mod arkalis_api {
    tonic::include_proto!("arkalis");
}

pub type ArkalisClient = ArkalisCoreServiceClient<InterceptedService<Channel, AuthInterceptor>>;

#[derive(Debug)]
pub struct Arkalis {
    pub client: ArkalisClient,
    pub url: String,
}

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

impl Arkalis {
    pub async fn new(arkalis_url: &str, token: &Option<String>) -> anyhow::Result<Arkalis> {
        let channel = Channel::from_shared(arkalis_url.to_owned())?
            .tls_config(ClientTlsConfig::new())?
            .connect()
            .await?;
        let client = ArkalisCoreServiceClient::with_interceptor(
            channel,
            AuthInterceptor {
                token: token.to_owned(),
            },
        );

        Ok(Arkalis {
            client,
            url: arkalis_url.to_owned(),
        })
    }
}
