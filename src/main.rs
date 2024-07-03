use leptos::*;
use leptos_dom::logging::console_log;

mod cosmos;

use cosmos::{
    Coin, DeliverTxResponse, Logger, StdFee, WalletManager, WalletRepo, ASSETS, CHAINS, WALLETS,
};

#[component]
fn App() -> impl IntoView {
    let clickHandler = move |_| {
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

            let client = walletRepo.current().get_signing_stargate_client().await;

            let result = client
                .sendTokens(
                    &walletRepo.current().address(),
                    "cosmos15aptdqmm7ddgtcrjvc5hs988rlrkze40l4q0he",
                    vec![serde_wasm_bindgen::to_value(&Coin {
                        denom: "uatom".to_owned(),
                        amount: "1".to_owned(),
                    })
                    .unwrap()],
                    serde_wasm_bindgen::to_value(&StdFee {
                        amount: vec![Coin {
                            denom: "uatom".to_owned(),
                            amount: "500".to_owned(),
                        }],
                        gas: "100000".to_owned(),
                    })
                    .unwrap(),
                )
                .await;

            let tx_hash = &serde_wasm_bindgen::from_value::<DeliverTxResponse>(result)
                .unwrap()
                .transaction_hash;

            console_log(format!("transactionHash: {}", tx_hash).as_str());
        });
    };

    view! {
        <button on:click=clickHandler>
            RUN
        </button>
    }
}

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App/> })
}
