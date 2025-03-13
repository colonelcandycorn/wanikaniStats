use governor::DefaultDirectRateLimiter;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use super::*;

const USER_URL: &str = "https://api.wanikani.com/v2/user";
const RESETS_URL: &str = "https://api.wanikani.com/v2/resets";
const REVIEW_STATS_URL: &str = "https://api.wanikani.com/v2/review_statistics";
const SUBJECT_URL: &str = "https://api.wanikani.com/v2/subjects";
const ASSIGNMENT_URL: &str = "https://api.wanikani.com/v2/assignments";

type ApiClientError = reqwest::Error;

impl<'a> ApiClient<'a> {
    pub fn new(
        token: String,
        client: &'a reqwest::Client, // change to arc
        limiter: &'a DefaultDirectRateLimiter, // change to arc
                                     // add cache for subjects
    ) -> Self {
        ApiClient {
            token,
            client,
            limiter,
        }
    }

    async fn get_response<T>(&self, url: &str) -> Result<ReqwestResponse<T>, ApiClientError>
    where
        T: DeserializeOwned,
    {
        Ok(self
            .get_response_with_params::<T, String>(url, None)
            .await?)
    }

    async fn get_response_with_params<T, K>(
        &self,
        url: &str,
        params: Option<Vec<(&str, K)>>,
    ) -> Result<ReqwestResponse<T>, ApiClientError>
    where
        T: DeserializeOwned,
        K: Serialize,
    {
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
    where
        T: DeserializeOwned,
    {
        self.limiter.until_ready().await;
        let raw = self.get_response::<Response<T>>(url).await?;
        let processed = self.raw_response_to_data(raw).await?;

        Ok(processed)
    }

    pub async fn get_all_pages_of_paged_data<T>(
        &self,
        paged_url: &str,
    ) -> Result<Vec<Response<T>>, ApiClientError>
    where
        T: DeserializeOwned,
    {
        Ok(self
            .get_all_pages_of_paged_data_with_params::<T, String>(paged_url, None)
            .await?)
    }

    pub async fn get_all_pages_of_paged_data_with_params<T, K>(
        &self,
        paged_url: &str,
        params: Option<Vec<(&str, K)>>,
    ) -> Result<Vec<Response<T>>, ApiClientError>
    where
        T: DeserializeOwned,
        K: Serialize,
    {
        self.limiter.until_ready().await;

        let raw = match params {
            Some(_) => {
                self.get_response_with_params::<PagedData<T>, K>(paged_url, params)
                    .await?
            }
            None => self.get_response::<PagedData<T>>(paged_url).await?,
        };
        let mut processed = self.raw_response_to_data(raw).await?;
        let mut result: Vec<Response<T>> = processed.data;

        while let Some(PageData {
            next_url: Some(ref url),
            ..
        }) = processed.pages
        {
            self.limiter.until_ready().await;
            let raw = self.get_response::<PagedData<T>>(&url).await?;
            processed = self.raw_response_to_data(raw).await?;

            result.append(&mut processed.data);
        }

        Ok(result)
    }

    pub fn get_list_of_subjects_to_request(
        &self,
        review_stats: &Vec<Response<ReviewStatistic>>,
        assignment_stats: &Vec<Response<Assignment>>,
    ) -> Vec<i32> {
        let mut result: HashSet<i32> = HashSet::new();

        for stat in review_stats {
            result.insert(stat.data.subject_id);
        }

        for assignment in assignment_stats {
            result.insert(assignment.data.subject_id);
        }

        result.into_iter().collect()
    }

    pub async fn construct_id_to_subject_hash(
        &self,
        subject_list: &Vec<i32>,
    ) -> Result<HashMap<i32, SubjectWithType>, ApiClientError> {
        let subject_list_strs: Vec<String> = subject_list
            .into_iter()
            .map(|subject| subject.to_string())
            .collect();
        let query_params = vec![("ids", subject_list_strs.join(","))];
        let all_subjects: Vec<Response<Subject>> = self
            .get_all_pages_of_paged_data_with_params(SUBJECT_URL, Some(query_params))
            .await?;
        let result: HashMap<i32, SubjectWithType> = all_subjects
            .into_iter()
            .map(|response| {
                let subject_type = match response.object.as_str() {
                    "radical" => SubjectType::Radical,
                    "kanji" => SubjectType::Kanji,
                    "vocabulary" => SubjectType::Vocabulary,
                    "kana_vocabulary" => SubjectType::KanaVocabulary,
                    _ => panic!("Unknown Subject Type"),
                };
                (
                    response.id.unwrap(),
                    SubjectWithType::new(response.data, subject_type),
                )
            })
            .collect();

        Ok(result)
    }

    pub async fn get_all_assignments(&self) -> Result<Vec<Response<Assignment>>, ApiClientError> {
        self.get_all_pages_of_paged_data(ASSIGNMENT_URL).await
    }

    pub async fn get_all_resets(&self) -> Result<Vec<Response<Reset>>, ApiClientError> {
        self.get_all_pages_of_paged_data(RESETS_URL).await
    }

    pub async fn get_all_review_stats(
        &self,
    ) -> Result<Vec<Response<ReviewStatistic>>, ApiClientError> {
        self.get_all_pages_of_paged_data(REVIEW_STATS_URL).await
    }

    pub async fn build_complete_user_info(
        &self,
    ) -> Result<CompleteUserInfo, Box<dyn std::error::Error>> {
        let user_data = self.get_user_data().await?;
        let review_data = self.get_all_review_stats().await?;
        let assignment_data = self.get_all_assignments().await?;
        let reset_data = self.get_all_resets().await?;
        let sub_vec = self.get_list_of_subjects_to_request(&review_data, &assignment_data);
        let hashy = self.construct_id_to_subject_hash(&sub_vec).await?;

        let builder = CompleteUserInfoBuilder::new(
            user_data,
            review_data
                .into_iter()
                .map(|response| response.data)
                .collect(),
            assignment_data
                .into_iter()
                .map(|response| response.data)
                .collect(),
            reset_data
                .into_iter()
                .map(|response| response.data)
                .collect(),
            hashy,
        );

        Ok(builder.build()?)
    }
}

impl SubjectWithType {
    pub fn new(subject: Subject, subject_type: SubjectType) -> Self {
        SubjectWithType {
            subject,
            subject_type,
        }
    }
}


