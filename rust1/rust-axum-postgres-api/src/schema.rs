use serde::{
    Serialize,
    Deserialize
};


#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserSchema {
    pub username: String,
    pub passcode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUserSchema {
    pub passcode: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateNoteSchema {
    pub title: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateNoteSchema {
    pub title: Option<String>,
    pub content: Option<String>,
    pub category: Option<String>,
    pub published: Option<bool>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct CreateSubscriptionSchema {
    pub user_username: String,
    pub subscriber_username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<String>
}