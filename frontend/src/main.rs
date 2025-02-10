use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let url = use_state(|| String::new());
    let message = use_state(|| String::new());

    let oninput = {
        let url = url.clone();
        move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            url.set(input.value());
        }
    };

    let onclick = {
        let url = url.clone();
        let message = message.clone();
        move |_| {
            let url = (*url).clone();
            let message = message.clone();

            spawn_local(async move {
                let request = serde_json::json!({
                    "url": url,
                });

                let response = reqwest::Client::new()
                    .post("http://localhost:8080/process-url")
                    .json(&request)
                    .send()
                    .await;

                match response {
                    Ok(res) => {
                        if res.status().is_success() {
                            message.set("URL enviada com sucesso!".to_string());
                        } else {
                            message.set("Erro ao enviar a URL.".to_string());
                        }
                    }
                    Err(_) => {
                        message.set("Falha na comunicação com o backend.".to_string());
                    }
                }
            });
        }
    };

    html! {
        <div>
        <h1>{"RustTube"}</h1>
        <input type="text" {oninput} placeholder="Cole a URL do YouTube aqui" />
        <button {onclick}>{"Enviar"}</button>
        <p>{ "URL digitada: " } { &*url }</p>
        <p>{ &*message }</p>
    </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
