use wanikani_stats::data_processing::api_client;
use wanikani_stats::data_processing::wanikani_data;
use reqwest;
use std::collections::HashMap;
use dotenvy;
use serde::{ Deserialize, Serialize };
use chrono::{ DateTime, Local, Utc };


#[derive(Deserialize, Serialize, Debug)]
struct User {
    level: i32,
    username: String,
    started_at: DateTime<Local>
}

#[derive(Deserialize, Serialize, Debug)]
struct PageData {
    per_page: i32,
    next_page: Option<String>,
    last_page: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct PaginatedData<T> {
    pages: PageData,
    data: T
}

#[derive(Deserialize, Serialize, Debug)]
struct Response<T> {
    pages: Option<PageData>,
    data: T
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenvy::dotenv()?;


    let wk_token = std::env::var("WK_TOKEN").expect("WK_TOKEN must be set");
    let test_url = "https://api.wanikani.com/v2/user";
    
    let response = reqwest::Client::new().get(test_url).header("Authorization", format!("Bearer {}", wk_token)).send().await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<Response<User>>().await {
                Ok(parsed) => {
                    println!("Parsed User: {:?}", parsed);
                },
                Err(e) => println!("Error processing user {}", e)
            }
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Invalid Token")
        }
        other => {
            panic!("{:?}", other)
        }
    }


    Ok(())
}
