use hyper::client::connect::HttpConnector;
use hyper::{client::ResponseFuture, Body, Client, Request, Response, Uri};
use hyper_openssl::HttpsConnector;
use openssl::{
    ssl::{SslConnector, SslMethod},
    x509::X509,
};
use std::{error::Error, task::Poll};
use tonic::body::BoxBody;
use tonic_openssl::ALPN_H2_WIRE;
use tower::Service;

pub mod autopilotrpc {
    tonic::include_proto!("autopilotrpc");
}

pub mod chainrpc {
    tonic::include_proto!("chainrpc");
}

pub mod devrpc {
    tonic::include_proto!("devrpc");
}

pub mod invoicesrpc {
    tonic::include_proto!("invoicesrpc");
}

pub mod lnrpc {
    tonic::include_proto!("lnrpc");
}

pub mod lnclipb {
    tonic::include_proto!("lnclipb");
}

pub mod neutrinorpc {
    tonic::include_proto!("neutrinorpc");
}

pub mod peersrpc {
    tonic::include_proto!("peersrpc");
}

pub mod routerrpc {
    tonic::include_proto!("routerrpc");
}

pub mod signrpc {
    tonic::include_proto!("signrpc");
}

pub mod verrpc {
    tonic::include_proto!("verrpc");
}

pub mod walletrpc {
    tonic::include_proto!("walletrpc");
}

pub mod watchtowerrpc {
    tonic::include_proto!("watchtowerrpc");
}

pub mod wtclientrpc {
    tonic::include_proto!("wtclientrpc");
}

/// [`tonic::Status`] is re-exported as `LndClientError` for convenience.
pub type LndClientError = tonic::Status;

pub type LndAutopilotClient = autopilotrpc::autopilot_client::AutopilotClient<
    tonic::codegen::InterceptedService<MyChannel, MacaroonInterceptor>,
>;

pub type LndChainClient = chainrpc::chain_notifier_client::ChainNotifierClient<
    tonic::codegen::InterceptedService<MyChannel, MacaroonInterceptor>,
>;

pub type LndDevClient = devrpc::dev_client::DevClient<
    tonic::codegen::InterceptedService<MyChannel, MacaroonInterceptor>,
>;

pub type LndInvoicesClient = invoicesrpc::invoices_client::InvoicesClient<
    tonic::codegen::InterceptedService<MyChannel, MacaroonInterceptor>,
>;

pub type LndLightningClient = lnrpc::lightning_client::LightningClient<
    tonic::codegen::InterceptedService<MyChannel, MacaroonInterceptor>,
>;

pub type LndStateClient = lnrpc::state_client::StateClient<
    tonic::codegen::InterceptedService<MyChannel, MacaroonInterceptor>,
>;

pub type LndWalletUnlockerClient = lnrpc::wallet_unlocker_client::WalletUnlockerClient<
    tonic::codegen::InterceptedService<MyChannel, MacaroonInterceptor>,
>;

pub type LndNeutrinoClient = neutrinorpc::neutrino_kit_client::NeutrinoKitClient<
    tonic::codegen::InterceptedService<MyChannel, MacaroonInterceptor>,
>;

pub type LndPeersClient = peersrpc::peers_client::PeersClient<
    tonic::codegen::InterceptedService<MyChannel, MacaroonInterceptor>,
>;

pub type LndRouterClient = routerrpc::router_client::RouterClient<
    tonic::codegen::InterceptedService<MyChannel, MacaroonInterceptor>,
>;

pub type LndSignerClient = signrpc::signer_client::SignerClient<
    tonic::codegen::InterceptedService<MyChannel, MacaroonInterceptor>,
>;

pub type LndVersionerClient = verrpc::versioner_client::VersionerClient<
    tonic::codegen::InterceptedService<MyChannel, MacaroonInterceptor>,
>;

pub type LndWalletClient = walletrpc::wallet_kit_client::WalletKitClient<
    tonic::codegen::InterceptedService<MyChannel, MacaroonInterceptor>,
>;

pub type LndWatchtowerClient = watchtowerrpc::watchtower_client::WatchtowerClient<
    tonic::codegen::InterceptedService<MyChannel, MacaroonInterceptor>,
>;

pub type LndWtcClient = wtclientrpc::watchtower_client_client::WatchtowerClientClient<
    tonic::codegen::InterceptedService<MyChannel, MacaroonInterceptor>,
>;

#[derive(Clone)]
pub struct LndClient {
    autopilot: LndAutopilotClient,
    chain: LndChainClient,
    dev: LndDevClient,
    invoices: LndInvoicesClient,
    lightning: LndLightningClient,
    state: LndStateClient,
    wallet_unlocker: LndWalletUnlockerClient,
    neutrino: LndNeutrinoClient,
    peers: LndPeersClient,
    router: LndRouterClient,
    signer: LndSignerClient,
    versioner: LndVersionerClient,
    wallet: LndWalletClient,
    watchtower: LndWatchtowerClient,
    wtc: LndWtcClient,
}

impl LndClient {
    pub fn autopilot(&mut self) -> &mut LndAutopilotClient {
        &mut self.autopilot
    }

    pub fn chain(&mut self) -> &mut LndChainClient {
        &mut self.chain
    }

    pub fn dev(&mut self) -> &mut LndDevClient {
        &mut self.dev
    }

    pub fn invoices(&mut self) -> &mut LndInvoicesClient {
        &mut self.invoices
    }

    pub fn lightning(&mut self) -> &mut LndLightningClient {
        &mut self.lightning
    }

    pub fn state(&mut self) -> &mut LndStateClient {
        &mut self.state
    }

    pub fn wallet_unlocker(&mut self) -> &mut LndWalletUnlockerClient {
        &mut self.wallet_unlocker
    }

    pub fn neutrino(&mut self) -> &mut LndNeutrinoClient {
        &mut self.neutrino
    }

    pub fn peers(&mut self) -> &mut LndPeersClient {
        &mut self.peers
    }

    pub fn router(&mut self) -> &mut LndRouterClient {
        &mut self.router
    }

    pub fn signer(&mut self) -> &mut LndSignerClient {
        &mut self.signer
    }

    pub fn versioner(&mut self) -> &mut LndVersionerClient {
        &mut self.versioner
    }

    pub fn wallet(&mut self) -> &mut LndWalletClient {
        &mut self.wallet
    }

    pub fn watchtower(&mut self) -> &mut LndWatchtowerClient {
        &mut self.watchtower
    }

    pub fn wtc(&mut self) -> &mut LndWtcClient {
        &mut self.wtc
    }
}

/// Supplies requests with macaroon
#[derive(Clone)]
pub struct MacaroonInterceptor {
    macaroon: String,
}

impl tonic::service::Interceptor for MacaroonInterceptor {
    fn call(
        &mut self,
        mut request: tonic::Request<()>,
    ) -> Result<tonic::Request<()>, LndClientError> {
        request.metadata_mut().insert(
            "macaroon",
            #[allow(deprecated)]
            tonic::metadata::MetadataValue::from_str(&self.macaroon)
                .expect("hex produced non-ascii"),
        );
        Ok(request)
    }
}

async fn get_channel(
    cert: String,
    socket: String,
) -> Result<MyChannel, Box<dyn std::error::Error>> {
    let lnd_address = format!("https://{}", socket).to_string();
    let pem = hex::decode(cert).expect("FailedToDecodeTlsCert");
    let uri = lnd_address.parse::<Uri>().unwrap();
    let channel = MyChannel::new(Some(pem), uri).await?;
    Ok(channel)
}

pub async fn connect(
    cert: String,
    macaroon: String,
    socket: String,
) -> Result<LndClient, Box<dyn std::error::Error>> {
    let channel = get_channel(cert, socket).await?;
    let interceptor = MacaroonInterceptor { macaroon };
    let client = LndClient {
        autopilot: autopilotrpc::autopilot_client::AutopilotClient::with_interceptor(
            channel.clone(),
            interceptor.clone(),
        ),
        chain: chainrpc::chain_notifier_client::ChainNotifierClient::with_interceptor(
            channel.clone(),
            interceptor.clone(),
        ),
        dev: devrpc::dev_client::DevClient::with_interceptor(channel.clone(), interceptor.clone()),
        invoices: invoicesrpc::invoices_client::InvoicesClient::with_interceptor(
            channel.clone(),
            interceptor.clone(),
        ),
        lightning: lnrpc::lightning_client::LightningClient::with_interceptor(
            channel.clone(),
            interceptor.clone(),
        ),
        state: lnrpc::state_client::StateClient::with_interceptor(
            channel.clone(),
            interceptor.clone(),
        ),
        wallet_unlocker: lnrpc::wallet_unlocker_client::WalletUnlockerClient::with_interceptor(
            channel.clone(),
            interceptor.clone(),
        ),
        neutrino: neutrinorpc::neutrino_kit_client::NeutrinoKitClient::with_interceptor(
            channel.clone(),
            interceptor.clone(),
        ),
        peers: peersrpc::peers_client::PeersClient::with_interceptor(
            channel.clone(),
            interceptor.clone(),
        ),
        router: routerrpc::router_client::RouterClient::with_interceptor(
            channel.clone(),
            interceptor.clone(),
        ),
        signer: signrpc::signer_client::SignerClient::with_interceptor(
            channel.clone(),
            interceptor.clone(),
        ),
        versioner: verrpc::versioner_client::VersionerClient::with_interceptor(
            channel.clone(),
            interceptor.clone(),
        ),
        wallet: walletrpc::wallet_kit_client::WalletKitClient::with_interceptor(
            channel.clone(),
            interceptor.clone(),
        ),
        watchtower: watchtowerrpc::watchtower_client::WatchtowerClient::with_interceptor(
            channel.clone(),
            interceptor.clone(),
        ),
        wtc: wtclientrpc::watchtower_client_client::WatchtowerClientClient::with_interceptor(
            channel.clone(),
            interceptor.clone(),
        ),
    };
    Ok(client)
}

#[derive(Clone)]
pub struct MyChannel {
    uri: Uri,
    client: MyClient,
}

#[derive(Clone)]
enum MyClient {
    ClearText(Client<HttpConnector, BoxBody>),
    Tls(Client<HttpsConnector<HttpConnector>, BoxBody>),
}

impl MyChannel {
    pub async fn new(certificate: Option<Vec<u8>>, uri: Uri) -> Result<Self, Box<dyn Error>> {
        let mut http = HttpConnector::new();
        http.enforce_http(false);
        let client = match certificate {
            None => MyClient::ClearText(Client::builder().http2_only(true).build(http)),
            Some(pem) => {
                let ca = X509::from_pem(&pem[..])?;
                let mut connector = SslConnector::builder(SslMethod::tls())?;
                connector.cert_store_mut().add_cert(ca)?;
                connector.set_alpn_protos(ALPN_H2_WIRE)?;
                let mut https = HttpsConnector::with_connector(http, connector)?;
                https.set_callback(|c, _| {
                    c.set_verify_hostname(false);
                    Ok(())
                });
                MyClient::Tls(Client::builder().http2_only(true).build(https))
            }
        };

        Ok(Self { client, uri })
    }
}

impl Service<Request<BoxBody>> for MyChannel {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = ResponseFuture;

    fn poll_ready(&mut self, _: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, mut req: Request<BoxBody>) -> Self::Future {
        let uri = Uri::builder()
            .scheme(self.uri.scheme().unwrap().clone())
            .authority(self.uri.authority().unwrap().clone())
            .path_and_query(req.uri().path_and_query().unwrap().clone())
            .build()
            .unwrap();
        *req.uri_mut() = uri;
        match &self.client {
            MyClient::ClearText(client) => client.request(req),
            MyClient::Tls(client) => client.request(req),
        }
    }
}
