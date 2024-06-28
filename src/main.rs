use leptos::*;
use leptos_dom::logging::console_log;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
struct SayOptions {
    #[wasm_bindgen(getter_with_clone, js_name = text)]
    pub _text: String,
}

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace = cowsay)]
    fn say(s: SayOptions) -> String;
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button
        on:click=move |_| {
                set_count.update(|count| *count += 1);

                console_log(&say(SayOptions {
                    _text: count().to_string(),
                }));
            }
        >
            value: {count}
        </button>
    }
}

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App/> })
}
