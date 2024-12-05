use rusqlite::{Connection};

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
        .get("https://api.nytimes.com/svc/mostpopular/v2/emailed/7.json")
        .query(&params)
        // .basic_auth(user_name, password)
        // .query(&query)
        .send()?;


    // let mut request = reqwest::blocking::get("https://www.yabla-dev.com")?;
    let mut body = String::new();
    request.read_to_string(&mut body)?;

    let mut body_: serde_json::Value = serde_json::from_str(&body).unwrap();
    for key in body_["results"][0].as_object().unwrap().keys() {
        println!("{}", key);
    }
    

    println!("Status: {}", request.status());
    println!("Headers: \n{:?}", request.headers());
    println!("Body: \n{:?}", body_["results"][0]["abstract"]);
    Ok(())
}

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() -> rusqlite::Result<()> {

    real_main().unwrap();
    // let conn = Connection::open_in_memory()?;

    // conn.execute(
    //     "CREATE TABLE person (
    //         id    INTEGER PRIMARY KEY,
    //         name  TEXT NOT NULL,
    //         data  BLOB
    //     )",
    //     (), // empty list of parameters.
    // )?;
    // let me = Person {
    //     id: 0,
    //     name: "Steven".to_string(),
    //     data: None,
    // };
    // conn.execute(
    //     "INSERT INTO person (name, data) VALUES (?1, ?2)",
    //     (&me.name, &me.data),
    // )?;

    // let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    // let person_iter = stmt.query_map([], |row| {
    //     Ok(Person {
    //         id: row.get(0)?,
    //         name: row.get(1)?,
    //         data: row.get(2)?,
    //     })
    // })?;

    // for person in person_iter {
    //     println!("Found person {:?}", person.unwrap());
    // }
    Ok(())
}