
use bson::{doc,Document};
use serde::{Serialize,Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub _id: bson::oid::ObjectId,
    pub data: String
}