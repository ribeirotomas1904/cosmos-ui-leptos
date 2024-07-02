use leptos::*;
use leptos_dom::logging::console_log;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/test.js")]
extern "C" {
    #[wasm_bindgen]
    fn hello();
}

// #[wasm_bindgen(module = "https://cdn.jsdelivr.net/npm/chain-registry@1.63.15/+esm")]
#[wasm_bindgen(module = "/bundle.js")]
extern "C" {
    #[derive(Clone)]
    type Chain;

    #[wasm_bindgen(js_name = chains)]
    static CHAINS: Vec<Chain>;

    #[derive(Clone)]
    type AssetList;

    #[wasm_bindgen(js_name = assets)]
    static ASSETS: Vec<AssetList>;
}

#[wasm_bindgen(module = "https://cdn.jsdelivr.net/npm/@cosmos-kit/keplr-extension@2.12.2/+esm")]
extern "C" {
    #[derive(Clone)]
    type Wallet;

    #[wasm_bindgen(js_name = wallets)]
    static WALLETS: Vec<Wallet>;
}

#[derive(Clone)]
#[wasm_bindgen]
pub struct Coin {
    #[wasm_bindgen(getter_with_clone)]
    pub denom: String,

    #[wasm_bindgen(getter_with_clone)]
    pub amount: String,
}

#[wasm_bindgen]
pub struct StdFee {
    #[wasm_bindgen(getter_with_clone)]
    pub amount: Vec<Coin>,

    #[wasm_bindgen(getter_with_clone)]
    pub gas: String,
}

#[wasm_bindgen(module = "https://cdn.jsdelivr.net/npm/@cosmjs/stargate@0.32.4/+esm")]
extern "C" {

    type SigningStargateClient;

    #[wasm_bindgen(method)]
    async fn getChainId(this: &SigningStargateClient) -> JsValue; // -> String

    #[wasm_bindgen(method)]
    async fn sendTokens(
        this: &SigningStargateClient,
        senderAddress: &str,
        recipientAddress: &str,
        amount: Vec<Coin>,
        fee: StdFee,
    ) -> JsValue;

    #[wasm_bindgen(static_method_of = SigningStargateClient)]
    async fn connectWithSigner(endpoint: &str, signer: OfflineSigner) -> JsValue; // SigningStargateClient
}

impl SigningStargateClient {
    async fn get_chain_id(&self) -> String {
        let js_value = self.getChainId().await;

        js_value.as_string().unwrap()
    }

    async fn connect_with_signer(endpoint: &str, signer: OfflineSigner) -> SigningStargateClient {
        let js_value = SigningStargateClient::connectWithSigner(endpoint, signer).await;

        js_value.into()
    }
}

#[wasm_bindgen(module = "https://cdn.jsdelivr.net/npm/@cosmos-kit/core@2.13.1/+esm")]
extern "C" {

    type Logger;

    #[wasm_bindgen(constructor)]
    fn new() -> Logger;

    type OfflineSigner;

    type ChainWalletBase;

    // TODO: this should be protected somehow, to only be used through another type safe method
    #[wasm_bindgen(method)]
    async fn getSigningStargateClient(this: &ChainWalletBase) -> JsValue; // -> SigningStargateClient

    #[wasm_bindgen(method)]
    async fn initOfflineSigner(this: &ChainWalletBase);

    // TODO: this should be protected somehow, to only be used through another type safe method
    #[wasm_bindgen(method, getter)]
    fn offlineSigner(this: &ChainWalletBase) -> JsValue; // -> OfflineSigner

    type WalletRepo;

    #[wasm_bindgen(method)]
    async fn connect(this: &WalletRepo, walletName: &str);

    #[wasm_bindgen(method, getter)]
    fn current(this: &WalletRepo) -> ChainWalletBase;

    type WalletManager;

    #[wasm_bindgen(constructor)]
    fn new(
        chains: Vec<Chain>,
        wallets: Vec<Wallet>,
        logger: Logger,
        throwErrors: bool,
        subscribeConnectEvents: Option<bool>,
        allowedCosmiframeParentOrigins: Option<Vec<String>>,
        assetLists: Vec<AssetList>,
    ) -> WalletManager;

    #[wasm_bindgen(method)]
    fn getWalletRepo(this: &WalletManager, chainName: &str) -> WalletRepo;
}

impl ChainWalletBase {
    async fn get_signing_stargate_client(&self) -> SigningStargateClient {
        let js_value = self.getSigningStargateClient().await;

        // TODO: assert this at runtime
        // if js_value.is_instance_of::<SigningStargateClient>() {
        //     panic!();
        // }

        js_value.into()
    }
}

#[component]
fn App() -> impl IntoView {
    hello();

    view! {
        <button
            on:click=move |_| {
                leptos::spawn_local(async {
                    let logger = Logger::new();

                    let wallet_manager = WalletManager::new(
                        CHAINS.to_vec(),
                        WALLETS.to_vec(),
                        logger,
                        false,
                        None,
                        None,
                        ASSETS.to_vec(),
                    );

                    let walletRepo: WalletRepo = wallet_manager.getWalletRepo("cosmoshubtestnet");
                    walletRepo.connect("keplr-extension").await;

                    walletRepo.current().initOfflineSigner().await;

                    let offline_signer: OfflineSigner = walletRepo.current().offlineSigner().into();

                    let client: SigningStargateClient = SigningStargateClient::connect_with_signer("wss://rpc.sentry-01.theta-testnet.polypore.xyz", offline_signer).await;

                    // let client = walletRepo.current().get_signing_stargate_client().await;

                    // console_log(client.get_chain_id().await.as_str());

                    let _result = client
                            .sendTokens(
                                "cosmos1qkgwhmya6ftv4y99xac3ldxh8jd9a49xyyf4ff",
                                "cosmos15aptdqmm7ddgtcrjvc5hs988rlrkze40l4q0he",
                                vec![Coin {
                                   denom: "uatom".to_owned(),
                                   amount: "1".to_owned(),
                                }],
                                StdFee {
                                   amount: vec![Coin {
                                       denom: "uatom".to_owned(),
                                       amount: "500".to_owned(),
                                    }],
                                   gas: "100000".to_owned(),
                                },
                            )
                            .await;

                });
            }
        >
            RUN
        </button>
    }
}

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App/> })
}
