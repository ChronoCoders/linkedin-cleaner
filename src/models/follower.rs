use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Connection {
    #[serde(rename = "First Name")]
    pub first_name: String,
    #[serde(rename = "Last Name")]
    pub last_name: String,
    #[serde(rename = "URL")]
    pub url: String,
    #[serde(rename = "Email Address")]
    pub email: String,
    #[serde(rename = "Company")]
    pub company: String,
    #[serde(rename = "Position")]
    pub position: String,
    #[serde(rename = "Connected On")]
    pub connected_on: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct FilterResult {
    pub person: Person,
    pub should_remove: bool,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub enum Person {
    Connection(Connection),
}
