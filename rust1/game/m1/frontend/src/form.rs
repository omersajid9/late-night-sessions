use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlInputElement;
use serde_json::json;

use crate::router::Route;


#[function_component]
pub fn Form() -> Html {

    let navigator = use_navigator().unwrap();


    web_sys::console::log_1(&format!("Button clicked with value: {}", "a".to_string()).into());


    //name
    let title_ref = NodeRef::default();
    let title_ref_outer = title_ref.clone();

    //price
    let content_ref = NodeRef::default();
    let content_ref_outer = content_ref.clone();

    //submit form data
    let onclick = Callback::from(move |_| {
        // gloo_console::log!("Button Clicked");
        let content = content_ref.cast::<HtmlInputElement>().unwrap();
        let title = title_ref.cast::<HtmlInputElement>().unwrap();
        // gloo_console::log!(name.value());

        wasm_bindgen_futures::spawn_local(async move {
            let product = json!({
                "title": title.value(),
                "content": content.value()
            });
    
            let client = reqwest::Client::new();
            let res = client.post("http://localhost:8000/api/notes")
                .json(&product)
                .send()
                .await;
        });
        navigator.push(&Route::Notes)
    });

    html! {
        <div class="container">
            <h2>{"Add a Note"} </h2>
            <div>
                <label for="title" class="">
                    {"Title"}
                    <input ref={title_ref_outer.clone()}
                        id="title"
                        class="formInput"
                        type="text"
                    />
                </label> <br /> <br />
                <label for="content" class="">
                {"Content"}
                <input ref={content_ref_outer.clone()}
                    id="content"
                    class="formInput"
                    type="text"
                />
            </label> <br /><br />
            <button {onclick} class="btn-primary">{"Add Note"} </button>
            </div>
            <hr />
        </div>
    }
}