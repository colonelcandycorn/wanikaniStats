use chrono::{DateTime, Local};
use governor::DefaultDirectRateLimiter;
use reqwest;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::{ HashMap, HashSet };
use std::error;
use std::fmt;
use std::marker::PhantomData;
use std::sync::Arc;

const USER_URL: &str = "https://api.wanikani.com/v2/user";
const RESETS_URL: &str = "https://api.wanikani.com/v2/resets";
const REVIEW_STATS_URL: &str = "https://api.wanikani.com/v2/review_statistics";
const SUBJECT_URL: &str = "https://api.wanikani.com/v2/subjects";
const ASSIGNMENT_URL: &str = "https://api.wanikani.com/v2/assignments";

type ApiClientError = reqwest::Error;


#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    level: i32,
    username: String,
    started_at: DateTime<Local>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PageData {
    per_page: i32,
    next_url: Option<String>,
    previous_url: Option<String>,
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
    meaning: Option<String>,
    primary: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Subject {
    characters: Option<String>,
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
    id: Option<i32>,
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
    limiter: DefaultDirectRateLimiter
}

impl<'a> ApiClient<'a> {
    pub fn new(token: String, client: &'a reqwest::Client, limiter: DefaultDirectRateLimiter) -> Self {
        ApiClient { token, client, limiter }
    }

    async fn get_response<T>(&self, url: &str) -> Result<ReqwestResponse<T>, ApiClientError>
    where
        T: DeserializeOwned,
    {
        Ok(self.get_response_with_params::<T, String>(url, None).await?)
    }

    async fn get_response_with_params<T, K>(&self, url: &str, params: Option<Vec<(&str, K)>>) -> Result<ReqwestResponse<T>, ApiClientError>
    where
        T: DeserializeOwned,
        K: Serialize {
            let mut response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.token));

        if let Some(valid_param) = params {
            response = response.query(&valid_param)
        }


        let response = response.send().await?;

        Ok(ReqwestResponse::<T> {
            raw_response: response.error_for_status()?,
            resource_type: PhantomData,
        })
        }

    async fn raw_response_to_data<T>(
        &self,
        raw_response: ReqwestResponse<T>,
    ) -> Result<T, ApiClientError>
    where
        T: DeserializeOwned,
    {
        let parsed = raw_response.raw_response.json::<T>().await?;

        Ok(parsed)
    }

    pub async fn get_user_data(&self) -> Result<User, ApiClientError> {
        let raw = self.get_response::<Response<User>>(USER_URL).await?;
        let processed = self.raw_response_to_data(raw).await?;

        Ok(processed.data)
    }

    pub async fn get_non_paginated_data<T>(&self, url: &str) -> Result<Response<T>, ApiClientError>
        where T: DeserializeOwned {
            self.limiter.until_ready().await;
            let raw = self.get_response::<Response<T>>(url).await?;
            let processed = self.raw_response_to_data(raw).await?;

            Ok(processed)
        }

    pub async fn get_all_pages_of_paged_data<T>(
        &self,
        paged_url: &str,
    ) -> Result<Vec<Response<T>>, ApiClientError>
        where T: DeserializeOwned
    {
        Ok(self.get_all_pages_of_paged_data_with_params::<T, String>(paged_url, None).await?)
    }

    pub async fn get_all_pages_of_paged_data_with_params<T, K>(
        &self,
        paged_url: &str,
        params: Option<Vec<(&str, K)>>
    ) -> Result<Vec<Response<T>>, ApiClientError>
        where T: DeserializeOwned,
        K: Serialize
    {
        self.limiter.until_ready().await;

        let raw = match params {
            Some(_) => self.get_response_with_params::<PagedData<T>, K>(paged_url, params).await?,
            None => self.get_response::<PagedData<T>>(paged_url).await?
        };
        let mut processed = self.raw_response_to_data(raw).await?;
        let mut result: Vec<Response<T>> = processed.data;

        println!("Paged Data: {:?} Total Count: {:?}", processed.pages, processed.total_count);

        while let Some(PageData {
            next_url: Some(ref url),
            ..
        }) = processed.pages
        {
            self.limiter.until_ready().await;
            let raw = self
                .get_response::<PagedData<T>>(&url)
                .await?;
            processed = self.raw_response_to_data(raw).await?;

            result.append(&mut processed.data);
        }

        Ok(result)
    }

    pub fn get_list_of_subjects_to_request(&self, review_stats: &Vec<Response<ReviewStatistic>>) -> Vec<i32> {
        let mut result: HashSet<i32> = HashSet::new();

        println!("Length of review_stat vec: {:?}", review_stats.len());

        for stat in review_stats {
            result.insert(stat.data.subject_id);
        }

        result.into_iter().collect()
    }

    pub async fn construct_id_to_subject_hash(&self, subject_list: &Vec<i32>) -> Result<HashMap<i32, Subject>, ApiClientError> {
        let subject_list_strs: Vec<String> = subject_list.into_iter().map(|subject| subject.to_string()).collect();
        let query_params = vec![("ids", subject_list_strs.join(","))];
        let all_subjects: Vec<Response<Subject>> = self.get_all_pages_of_paged_data_with_params(SUBJECT_URL, Some(query_params)).await?;
        let result: HashMap<i32, Subject> = all_subjects.into_iter().map( |response| (response.id.unwrap(), response.data)).collect();

        Ok(result)
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
