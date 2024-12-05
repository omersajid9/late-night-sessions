// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::io::Read;
use error_chain::error_chain;
use reqwest::blocking::Client;
use serde_json::json;


error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}


fn real_main() -> Result<String>
{
    let client = Client::new();

    let api_key = "vThTdv1WxuB2Kdm52VGwfwRDFNT6F8Jn";

    let params = json!({
        "api-key": api_key
    });

    let mut request = client
        .get("https://api.nytimes.com/svc/mostpopular/v2/emailed/7.json")
        .query(&params)
        // .basic_auth(user_name, password)
        // .query(&query)
        .send()?;


    // let mut request = reqwest::blocking::get("https://www.yabla-dev.com")?;
    let mut body = String::new();
    request.read_to_string(&mut body)?;
    Ok(body)
    // let mut body_: serde_json::Value = serde_json::from_str(&body).unwrap();
    // for key in body_["results"][0].as_object().unwrap().keys() {
    //     println!("{}", key);
    // }
    
    // println!("Status: {}", request.status());
    // println!("Headers: \n{:?}", request.headers());
    // println!("Body: \n{:?}", body_["results"][0]["abstract"]);
    // Ok(body_)
}


use doe::clipboard::get_clipboard;

#[tauri::command]
fn greet(_name: &str) -> String {
  real_main().unwrap()
  //  format!("Ye kya hoa, {}", get_clipboard().unwrap())
}


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}