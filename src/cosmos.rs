// TODO: wrap all weakly typed functions in better typed functions and expose only the latter

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Coin {
    pub denom: String,
    pub amount: String,
}

#[derive(Serialize, Deserialize)]
pub struct StdFee {
    pub amount: Vec<Coin>,
    pub gas: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeliverTxResponse {
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
}

#[wasm_bindgen(module = "/bundle.js")]
extern "C" {
    #[derive(Clone)]
    pub type Chain;

    #[wasm_bindgen(js_name = chains)]
    pub static CHAINS: Vec<Chain>;

    #[derive(Clone)]
    pub type AssetList;

    #[wasm_bindgen(js_name = assets)]
    pub static ASSETS: Vec<AssetList>;

    #[derive(Clone)]
    pub type Wallet;

    #[wasm_bindgen(js_name = wallets)]
    pub static WALLETS: Vec<Wallet>;

    pub type SigningStargateClient;

    #[wasm_bindgen(method)]
    pub async fn getChainId(this: &SigningStargateClient) -> JsValue; // -> String

    #[wasm_bindgen(method)]
    pub async fn sendTokens(
        this: &SigningStargateClient,
        senderAddress: &str,
        recipientAddress: &str,
        amount: Vec<JsValue>, // Coin
        fee: JsValue,         // StdFee
    ) -> JsValue; // DeliverTxResponse

    #[wasm_bindgen(static_method_of = SigningStargateClient)]
    pub async fn connectWithSigner(endpoint: &str, signer: OfflineSigner) -> JsValue; // SigningStargateClient

    pub type Logger;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Logger;

    pub type OfflineSigner;

    pub type ChainWalletBase;

    #[wasm_bindgen(method, getter)]
    pub fn address(this: &ChainWalletBase) -> String;

    #[wasm_bindgen(method)]
    pub async fn getSigningStargateClient(this: &ChainWalletBase) -> JsValue; // -> SigningStargateClient

    #[wasm_bindgen(method)]
    pub async fn initOfflineSigner(this: &ChainWalletBase);

    #[wasm_bindgen(method, getter)]
    pub fn offlineSigner(this: &ChainWalletBase) -> JsValue; // -> OfflineSigner

    pub type WalletRepo;

    #[wasm_bindgen(method)]
    pub async fn connect(this: &WalletRepo, walletName: &str);

    #[wasm_bindgen(method, getter)]
    pub fn current(this: &WalletRepo) -> ChainWalletBase;

    pub type WalletManager;

    #[wasm_bindgen(constructor)]
    pub fn new(
        chains: Vec<Chain>,
        wallets: Vec<Wallet>,
        logger: Logger,
        throwErrors: bool,
        subscribeConnectEvents: Option<bool>,
        allowedCosmiframeParentOrigins: Option<Vec<String>>,
        assetLists: Vec<AssetList>,
    ) -> WalletManager;

    #[wasm_bindgen(method)]
    pub fn getWalletRepo(this: &WalletManager, chainName: &str) -> WalletRepo;
}

impl SigningStargateClient {
    pub async fn get_chain_id(&self) -> String {
        let js_value = self.getChainId().await;

        js_value.as_string().unwrap()
    }

    pub async fn connect_with_signer(
        endpoint: &str,
        signer: OfflineSigner,
    ) -> SigningStargateClient {
        let js_value = SigningStargateClient::connectWithSigner(endpoint, signer).await;

        js_value.into()
    }
}

impl ChainWalletBase {
    pub async fn get_signing_stargate_client(&self) -> SigningStargateClient {
        let js_value = self.getSigningStargateClient().await;

        // TODO: assert this at runtime
        // if js_value.is_instance_of::<SigningStargateClient>() {
        //     panic!();
        // }

        js_value.into()
    }
}
