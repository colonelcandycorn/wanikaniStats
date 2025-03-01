use chrono::{DateTime, Local, Utc};
use dotenvy;
use reqwest;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wanikani_stats::data_processing::api_client;
use wanikani_stats::data_processing::wanikani_data;
use std::fmt;

#[derive(Deserialize, Serialize, Debug)]
struct User {
    level: i32,
    username: String,
    started_at: DateTime<Local>,
}

#[derive(Deserialize, Serialize, Debug)]
struct PageData {
    per_page: i32,
    next_page: Option<String>,
    last_page: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Reset {
    created_at: DateTime<Local>,
    confirmed_at: DateTime<Local>,
    original_level: i32,
    target_level: i32,
}

#[derive(Deserialize, Serialize, Debug)]
struct VecData<T> {
    pages: Option<PageData>,
    data: Vec<Response<T>>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Response<T> {
    data: T,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let wk_token = std::env::var("WK_TOKEN").expect("WK_TOKEN must be set");
    let test_user_url = "https://api.wanikani.com/v2/user";
    let resets_url = "https://api.wanikani.com/v2/resets";

    let client = reqwest::Client::new();

    type ResetStuff = VecData<Reset>;

    test_api::<Response<User>>(test_user_url, &wk_token, &client).await.unwrap();
    test_api::<ResetStuff>(&resets_url, &wk_token, &client).await.unwrap();

    

    Ok(())
}

async fn test_api<T>(url: &str, token: &str, client: &reqwest::Client) -> Result<(), Box<dyn std::error::Error>>
where
    T: DeserializeOwned + fmt::Debug,
{
    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

        match response.status() {
            reqwest::StatusCode::OK => match response.json::<T>().await {
                Ok(parsed) => {
                    println!("Parsed User: {:?}", parsed);
                }
                Err(e) => println!("Error processing user {}", e),
            },
            reqwest::StatusCode::UNAUTHORIZED => {
                println!("Invalid Token")
            }
            other => {
                panic!("{:?}", other)
            }
        }


    Ok(())
}
