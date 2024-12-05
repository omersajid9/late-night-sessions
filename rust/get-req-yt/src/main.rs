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

fn main() 
{
    match real_main() 
    {
        Ok(()) => std::process::exit(0),
        Err(_) => std::process::exit(-1)
    }
}

fn real_main() -> Result<()>
{
    let client = Client::new();

    // let user_name = "yabla".to_string();
    // let password: Option<String> = Some("miguelito".to_string()); // Make sure to replace "password" with your actual password

    // let query = json!({
    //     "action": "instantsearch_videos",
    //     "term": "circo in",
    //     "field": "title"
    // });

    let api_key = "vThTdv1WxuB2Kdm52VGwfwRDFNT6F8Jn";

    let params = json!({
        "api-key": api_key
    });

    let mut request = client
        .get("https://api.nytimes.com/svc/archive/v1/2024/3.json")
        .query(&params)
        // .basic_auth(user_name, password)
        // .query(&query)
        .send()?;


    // let mut request = reqwest::blocking::get("https://www.yabla-dev.com")?;
    let mut body = String::new();
    request.read_to_string(&mut body)?;

    println!("Status: {}", request.status());
    println!("Headers: \n{:?}", request.headers());
    println!("Body: \n{}", body);
    Ok(())
}