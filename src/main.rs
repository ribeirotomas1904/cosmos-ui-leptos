use leptos::*;
use leptos_dom::logging::console_log;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::*;

#[wasm_bindgen(module = "https://cdn.jsdelivr.net/npm/chain-registry@1.63.15/+esm")]
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

#[wasm_bindgen(module = "https://cdn.jsdelivr.net/npm/@cosmos-kit/core@2.13.1/+esm")]
extern "C" {

    type Logger;

    #[wasm_bindgen(constructor)]
    fn new() -> Logger;

    type SigningStargateClient;

    #[wasm_bindgen(method)]
    async fn getChainId(this: &SigningStargateClient) -> JsValue; // -> String

    // #[wasm_bindgen(method)]
    // async fn sendTokens(
    //     this: &SigningStargateClient,
    //     senderAddress: &str,
    //     recipientAddress: &str,
    // ) -> JsValue;

    type ChainWalletBase;

    #[wasm_bindgen(method)]
    async fn getSigningStargateClient(this: &ChainWalletBase) -> JsValue; // -> SigningStargateClient

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

#[component]
fn App() -> impl IntoView {
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

        let client = walletRepo.current().getSigningStargateClient().await;

        let get_chain_id = Reflect::get(&client, &JsValue::from("getChainId")).unwrap();

        let result = Function::from(get_chain_id).call0(&client).unwrap();

        let cb = Closure::new(|chainId: JsValue| {
            console_log(&chainId.as_string().unwrap());
        });

        let js_promise = Promise::from(result).then(&cb);

        JsFuture::from(js_promise).await.unwrap();
    });

    view! {}
}

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App/> })
}
