use aws_config::{default_provider::region, Region};
use axum::{
    extract::Query, response::IntoResponse, routing::{get,post}, Router, http::{StatusCode}
};
use bson::{doc,Document};
use mongodb::{Client, options::{ClientOptions,FindOptions}, Collection};
use s3::primitives::ByteStream;
use serde::{Deserialize, Serialize};
use std::{any, error::Error, path::Path};
use futures_util::stream::TryStreamExt;
use aws_sdk_s3 as s3;
use std::{fs::File,io::Read};

pub mod constant;
pub mod structs;
use crate::{constant::*,structs::*};


#[tokio::main]
async fn main() {
    let app: Router = Router::new()
        .route("/", get(hello_word))
        .route("/path", get(|| async { "Path in the woods" }))
        .route("/add", post(add_document))
        .route("/find", get(find_all))
        .route("/image", post(image_upload))
        .route("/all_images", get(get_all_images))
        // .route("/greet", post(greet_query));
        ;


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn set_env()
{
    std::env::set_var("AWS_ACCESS_KEY_ID", "AKIAWHNRLJA2S6JF5OWK");
    std::env::set_var("AWS_SECRET_ACCESS_KEY", "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");
    std::env::set_var("AWS_REGION", "us-east-1");
}

async fn connect_s3() -> Result<aws_sdk_s3::Client, Box<dyn Error>>
{
    set_env();
    let region = Region::new("us-east-1");
    let config = aws_config::from_env().region(region).load().await;
    let client = aws_sdk_s3::Client::new(&config);
    Ok(client)
}

async fn get_images_s3() -> Result<(), Box<dyn Error>>
{
    let client = connect_s3().await?;
    let mut response = client
        .list_objects_v2()
        .bucket(S3_BUCKET_NAME)
        .max_keys(10) // In this example, go 10 at a time.
        .into_paginator()
        .send();

    while let Some(result) = response.next().await {
        match result {
            Ok(output) => {
                for object in output.contents() {
                    println!(" - {}", object.key().unwrap_or("Unknown"));
                }
            }
            Err(err) => {
                eprintln!("{err:?}")
            }
        }
    }

    Ok(())
}

async fn get_all_images() -> (StatusCode, String) {
    match get_images_s3().await {
        Ok(_) => (StatusCode::OK, "all images fetched".to_string()),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }

}

async fn image_to_s3() -> Result<(), Box<dyn Error>>
{
    let client = connect_s3().await?;

    let key = "my-image1.jpg";

    let body = ByteStream::from_path(Path::new("acid.jpg")).await?;
    let resp = client
        .put_object()
        .bucket(S3_BUCKET_NAME)
        .key(key)
        .body(body)
        .send()
        .await.unwrap();

    println!("Response image saved: {:?}", resp);

    Ok(())
}

async fn image_upload() -> (StatusCode, String)
{
    match image_to_s3().await {
        Ok(_) => (StatusCode::OK, "image_uploaded".to_string()),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }
    
}

async fn insert_to_db(message: String) -> Result<(), Box<dyn Error>>
{
    let doc = Data {
        _id: bson::oid::ObjectId::new(),
        data: message,
    };

    let collection = get_collection().await?;
    collection.insert_one(doc, None).await?;
    Ok(())
}

async fn find_db() -> Result<Vec<Data>, Box<dyn Error>>
{
    let mut result: Vec<Data> = Vec::new();
    let collection = get_collection().await?;
    // collection.find(filter, options);
        let find_query = doc!{ "data": "new"};
        // let find_options = FindOptions::builder().sort(doc! {"message": 1}).build();
        let mut cursor = collection.find(None, None).await?;
    
        while let Some(doc) = cursor.try_next().await? {
            let data = Data {
                _id: doc._id,
                data: doc.data,
            };
            println!("Object retrieved: {:?}", data);
            result.push(data);
        }
    Ok(result)
}

async fn find_all() -> (StatusCode, String)
{
    match find_db().await {
        Ok(_) => (StatusCode::OK, "Messages retrieved".to_string()),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }
}



async fn add_document(Query(params): Query<QueryParameters>) -> (StatusCode, String)
{
    let message = params.message.unwrap_or_default();
    match insert_to_db(message).await {
        Ok(_) => (StatusCode::CREATED, "Message added".to_string()),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }
}


async fn hello_word() -> &'static str {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    client_options.app_name = Some(APP_NAME.to_string());
    let client = Client::with_options(client_options).unwrap();

    let mut ss = String::new();
    for db_name in client.list_database_names(None, None).await.unwrap() {
        println!("{}", db_name);
        ss.push_str(&db_name);
        ss.push_str(", ");
    }

    Box::leak(ss.into_boxed_str())
}

async fn get_collection() -> Result<Collection<Data>, Box<dyn Error>>
{
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    client_options.app_name = Some(APP_NAME.to_string());
    let client = Client::with_options(client_options).unwrap();

    let database = client.database(DATABASE_NAME);
    let collection: Collection<Data> = database.collection(CHALLENGES_COLLECTION_NAME);
    Ok(collection)
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct QueryParameters {
    message: Option<String>
}





