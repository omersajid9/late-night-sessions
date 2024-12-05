
use serde_json::json;
use uuid::Uuid;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use std::{borrow::BorrowMut, fmt::Write, rc::Rc}; // Add this import at the top

use crate::{notes::Note, router::Route};


#[derive(Properties, PartialEq)]
pub struct NoteProps {
    pub note_id: Uuid
}

#[function_component]
pub fn EditNote(props: &NoteProps) -> Html {
    // let title = title_ref.cast::<HtmlInputElement>().unwrap();
    // title.set_value(&props.title.to_owned());
    let note_id = props.note_id;

    let navigator = use_navigator().unwrap();

    //name
    let title_ref = NodeRef::default();
    let title_ref_outer = title_ref.clone();



    //price
    let content_ref = NodeRef::default();
    let content_ref_outer = content_ref.clone();

    wasm_bindgen_futures::spawn_local(async move {
        let fetched_data = reqwest::get(format!("http://localhost:8000/api/notes/{}", note_id))
           .await
           .expect("cannot get data from url")
           .json::<Note>()
           .await
           .expect("cannot convert to json");
        let content = content_ref_outer.cast::<HtmlInputElement>().unwrap();
        content.set_value(&fetched_data.content);
        let title = title_ref_outer.cast::<HtmlInputElement>().unwrap();
        title.set_value(&fetched_data.title);

    });

    let title_ref_outer = title_ref.clone();
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
            let res = client.patch(format!("http://localhost:8000/api/notes/{}", note_id))
                .json(&product)
                .send()
                .await;
        });
        navigator.push(&Route::Notes)
    });

    html! {
        <div class="container">
            <h2>{"Edit your Note"} </h2>
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
            <button {onclick} class="btn-primary">{"Edit Note"} </button>
            </div>
            <hr />
        </div>
    }
}