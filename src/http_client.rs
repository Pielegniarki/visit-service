use serde::{Serialize, Deserialize};

use crate::models::DatabaseId;

pub struct HttpClient {
    http: reqwest::Client
}

const AUTHORIZATION_SERVICE: &'static str = "http://localhost:4003";

#[derive(Serialize)]
struct GetIdParams<'a> {
    token: &'a str
} 

#[derive(Deserialize)]
struct GetIdResponse {
    id: Option<DatabaseId>
} 

impl HttpClient {
    pub fn new() -> Self {
        Self {
            http: reqwest::Client::new()
        }
    }

    pub async fn get_id(&self, token: &str) -> Result<Option<DatabaseId>, reqwest::Error> {
        let resp = self
            .http
            .post(format!("{}/getId", AUTHORIZATION_SERVICE))
            .json(&GetIdParams { token })
            .send()
            .await?;

        let obj = resp.json::<GetIdResponse>().await?;
        
        Ok(obj.id)
    }
}