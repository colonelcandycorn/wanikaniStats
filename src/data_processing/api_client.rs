use chrono::{DateTime, Local};
use reqwest;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::marker::PhantomData;

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    level: i32,
    username: String,
    started_at: DateTime<Local>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PageData {
    per_page: i32,
    next_page: Option<String>,
    last_page: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ReviewStatistic {
    created_at: DateTime<Local>,
    meaning_correct: i32,
    meaning_current_streak: i32,
    meaning_incorrect: i32,
    meaning_max_streak: i32,
    percentage_correct: i32,
    reading_correct: i32,
    reading_current_streak: i32,
    reading_incorrect: i32,
    reading_max_streak: i32,
    subject_id: i32,
    subject_type: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Meanings {
    meaning: String,
    primary: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Subject {
    characters: String,
    level: i32,
    spaced_repetition_system_id: i32,
    meanings: Vec<Meanings>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Assignment {
    created_at: Option<DateTime<Local>>,
    passed_at: Option<DateTime<Local>>,
    srs_stage: i32,
    subject_id: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Reset {
    created_at: DateTime<Local>,
    confirmed_at: DateTime<Local>,
    original_level: i32,
    target_level: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PagedData<T> {
    pages: Option<PageData>,
    total_count: i32,
    data: Vec<Response<T>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Response<T> {
    data: T,
}

pub struct ReqwestResponse<T> {
    raw_response: reqwest::Response,
    resource_type: PhantomData<T>,
}

#[derive(Debug)]
pub struct ApiClient<'a> {
    token: String,
    client: &'a reqwest::Client,
}

impl<'a> ApiClient<'a> {
    pub fn new(token: String, client: &'a reqwest::Client) -> Self {
        ApiClient { token, client }
    }

    pub async fn get_response<T>(&self, url: &str) -> Result<ReqwestResponse<T>, reqwest::Error>
    where
        T: DeserializeOwned,
    {
        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?;

        Ok(ReqwestResponse::<T> {
            raw_response: response.error_for_status()?,
            resource_type: PhantomData,
        })
    }

    pub async fn raw_response_to_data<T>(
        &self,
        raw_response: ReqwestResponse<T>,
    ) -> Result<T, serde_json::Error>
    where
        T: DeserializeOwned,
    {
        let parsed = raw_response
            .raw_response
            .json::<T>()
            .await
            .map_err(serde::de::Error::custom)?;

        Ok(parsed)
    }
}

#[allow(unused)]
async fn test_api<T>(
    url: &str,
    token: &str,
    client: &reqwest::Client,
) -> Result<(), Box<dyn std::error::Error>>
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
