use reqwest::Client;
use serde::{Deserialize,Serialize};
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Note {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub category: Option<String>,
    pub published: Option<bool>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Properties, PartialEq)]
pub struct NoteUlProps {
    #[prop_or_default]
    pub id: Uuid,
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub content: String,
}

#[function_component]
pub fn NoteUl(props: &NoteUlProps) -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = {
        let id_clone = props.id.clone();

        Callback::from(move |_: MouseEvent| {
            let id_clone = id_clone.clone(); // Clone again for async block
            web_sys::console::log_1(&format!("Clicked").into());

            // Spawn an asynchronous task to fetch data
            wasm_bindgen_futures::spawn_local(async move {
                let client = Client::new();
                let url = format!("http://localhost:8000/api/notes/{}", id_clone);

                // Send the DELETE request
                let response = client
                    .delete(&url)
                    .send()
                    .await;
                if response.is_err() {
                    web_sys::console::log_1(&format!("DELETE request failed with status code").into());
                } else {
                    web_sys::console::log_1(&format!("DELETE request successful").into());
                }
                        
                // let fetched_data = reqwest::delete(format!("http://localhost:8000/api/notes/{}", id_clone))
                //     .await
                //     .expect("could not get notes data");
                // Handle fetched data here
            });
        })
    };

    html! {
        <li key={format!("{}", props.id)} class={classes!("linky")}  ><Link<Route>  to={Route::EditNote {note_id: props.id }}><span class={classes!("linky")}>{format!("Title: {}, Content: {}", props.title.clone(), props.content.clone()) }</span></Link<Route>> <span onclick={ onclick }>{"    x"}</span></li>
    }

}

#[function_component]
pub fn Notes() -> Html {

    let data: UseStateHandle<Vec<Note>> = use_state(|| vec![]);
    let loading = use_state(|| true);

    {
        let data_clone = data.clone();
        let loading_clone = loading.clone();

        use_effect(move || {
            wasm_bindgen_futures::spawn_local(async move {
                
                let fetched_data = reqwest::get("http://localhost:8000/api/notes")
                    .await
                    .expect("could not get notes data")
                    .json::<Vec<Note>>()
                    .await
                    .expect("could not convert notes data to json");
                data_clone.set(fetched_data);
                loading_clone.set(false); // Set loading to false after data is fetched
            });
            || ()
        })
    }

    let loader = if *loading {
        html! { <div class="loader">{"Loading..."}</div> }
    } else {
        html! {}
    };

    // let onclick = {
    //     let onclick_callback = Callback::from(move |_| {
    //         web_sys::console::log_1(&format!("Button clicked with value: {}", value).into());
    //     });
    //     onclick_callback
    // };


    let notes = data.iter().map(|note|
         html!{
        <ul>
            <NoteUl id={note.id} title={note.title.clone()} content={note.content.clone()}/>
        </ul>
    }).collect::<Html>();
    html! {
        <div class="container">
            <button class="btn-primary">
                <Link<Route> to={Route::AddProduct} >{ "Add new Note" }</Link<Route>>
            </button>
            <h2>{"Your Notes: "} {data.len()} </h2>
            {loader}
            <p>{notes}</p>
        </div>

    }
}

use yew::prelude::*;

#[function_component]
pub fn Products() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        web_sys::console::log_1(&format!("B").into());
        let counter = counter.clone();
        Callback::from(move |_| {
            web_sys::console::log_1(&format!("A").into());
            counter.set(*counter + 1)})
    };
            web_sys::console::log_1(&format!("A").into());


    html! {
        <div>
            <button {onclick}>{ "Increment value" }</button>
            <p>
                <b>{ "Current value: " }</b>
                { *counter }
            </p>
        </div>
    }
}
