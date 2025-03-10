use chrono::{DateTime, Local};
use governor::DefaultDirectRateLimiter;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error;
use std::fmt;
use std::marker::PhantomData;

const USER_URL: &str = "https://api.wanikani.com/v2/user";
const RESETS_URL: &str = "https://api.wanikani.com/v2/resets";
const REVIEW_STATS_URL: &str = "https://api.wanikani.com/v2/review_statistics";
const SUBJECT_URL: &str = "https://api.wanikani.com/v2/subjects";
const ASSIGNMENT_URL: &str = "https://api.wanikani.com/v2/assignments";

type ApiClientError = reqwest::Error;

#[derive(Debug, Clone)]
pub struct CompleteUserInfo {
    user: User,
    review_stats: Vec<ReviewStatistic>,
    assignments: Vec<Assignment>,
    resets: Vec<Reset>,
    id_to_subjects: HashMap<i32, SubjectWithType>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SubjectType {
    KanaVocabulary,
    Kanji,
    Radical,
    Vocabulary,
}

#[derive(Debug)]
pub struct MissingSubjectError;

impl fmt::Display for MissingSubjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Missing Subject")
    }
}

impl error::Error for MissingSubjectError {}

#[derive(Debug)]
pub struct SubjectTypeStats {
    pub subject_type: SubjectType,
    pub num_of_meaning_correct: i32,
    pub num_of_meaning_incorrect: i32,
    pub num_of_reading_correct: i32,
    pub num_of_reading_incorrect: i32,
}

impl CompleteUserInfo {
    pub fn pretty_print(&self) {
        println!("User: {}", self.get_user_name());
        println!("Level: {}", self.get_level());
        println!("Started At: {}", self.get_started_at());
        println!("Number of Resets: {}", self.get_num_of_resets());
        println!(
            "Most Recent Reset: {:?}",
            self.get_date_of_most_recent_reset()
        );
        println!(
            "Number of Passed Radicals: {}",
            self.get_num_of_passed(SubjectType::Radical).unwrap()
        );
        println!(
            "Number of Passed Kanji: {}",
            self.get_num_of_passed(SubjectType::Kanji).unwrap()
        );
        println!(
            "Number of Passed Vocabulary: {}",
            self.get_num_of_passed(SubjectType::Vocabulary).unwrap()
        );
        println!(
            "Number of Passed Kana Vocabulary: {}",
            self.get_num_of_passed(SubjectType::KanaVocabulary).unwrap()
        );
        println!(
            "Radical Stats: {:?}",
            self.get_subject_type_stats(SubjectType::Radical).unwrap()
        );
        println!(
            "Kanji Stats: {:?}",
            self.get_subject_type_stats(SubjectType::Kanji).unwrap()
        );
        println!(
            "Vocabulary Stats: {:?}",
            self.get_subject_type_stats(SubjectType::Vocabulary)
                .unwrap()
        );
        println!(
            "Kana Vocabulary Stats: {:?}",
            self.get_subject_type_stats(SubjectType::KanaVocabulary)
                .unwrap()
        );
    }
    pub fn get_user_name(&self) -> &str {
        &self.user.username
    }

    pub fn get_level(&self) -> i32 {
        self.user.level
    }

    pub fn get_num_of_resets(&self) -> i32 {
        self.resets.len() as i32
    }

    pub fn get_date_of_most_recent_reset(&self) -> Option<&DateTime<Local>> {
        self.resets.iter().map(|reset| &reset.confirmed_at).max()
    }

    pub fn get_started_at(&self) -> &DateTime<Local> {
        &self.user.started_at
    }

    pub fn get_num_of_passed(&self, subject: SubjectType) -> Result<i32, MissingSubjectError> {
        let mut result = 0;

        for assignment in &self.assignments {
            let subject_obj = self.id_to_subjects.get(&assignment.subject_id);

            if subject_obj.is_none() {
                return Err(MissingSubjectError);
            }

            let subject_type = &subject_obj.unwrap().subject_type;

            if *subject_type == subject && assignment.passed_at.is_some() {
                result += 1;
            }
        }

        Ok(result)
    }

    pub fn get_subject_type_stats(
        &self,
        subject: SubjectType,
    ) -> Result<SubjectTypeStats, MissingSubjectError> {
        let mut meaning_correct = 0;
        let mut meaning_incorrect = 0;
        let mut reading_correct = 0;
        let mut reading_incorrect = 0;

        for review_stat in &self.review_stats {
            let subject_obj = self.id_to_subjects.get(&review_stat.subject_id);

            if subject_obj.is_none() {
                return Err(MissingSubjectError);
            }

            let subject_type = &subject_obj.unwrap().subject_type;

            if *subject_type == subject {
                meaning_correct += review_stat.meaning_correct;
                meaning_incorrect += review_stat.meaning_incorrect;
                reading_correct += review_stat.reading_correct;
                reading_incorrect += review_stat.reading_incorrect;
            }
        }

        Ok(SubjectTypeStats {
            subject_type: subject,
            num_of_meaning_correct: meaning_correct,
            num_of_meaning_incorrect: meaning_incorrect,
            num_of_reading_correct: reading_correct,
            num_of_reading_incorrect: reading_incorrect,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
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

#[derive(Deserialize, Serialize, Debug, Clone)]
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Meanings {
    meaning: Option<String>,
    primary: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Subject {
    characters: Option<String>,
    level: i32,
    spaced_repetition_system_id: i32,
    meanings: Vec<Meanings>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Assignment {
    created_at: Option<DateTime<Local>>,
    passed_at: Option<DateTime<Local>>,
    srs_stage: i32,
    subject_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
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
    object: String,
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
    limiter: &'a DefaultDirectRateLimiter,
}

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

    pub async fn build_complete_user_info(&self) -> Result<CompleteUserInfo, ApiClientError> {
        let user_data = self.get_user_data().await?;
        let review_data = self.get_all_review_stats().await?;
        let assignment_data = self.get_all_assignments().await?;
        let reset_data = self.get_all_resets().await?;
        let sub_vec = self.get_list_of_subjects_to_request(&review_data, &assignment_data);
        let hashy = self.construct_id_to_subject_hash(&sub_vec).await?;

        Ok(CompleteUserInfo {
            user: user_data,
            review_stats: review_data
                .into_iter()
                .map(|response| response.data)
                .collect(),
            assignments: assignment_data
                .into_iter()
                .map(|response| response.data)
                .collect(),
            resets: reset_data
                .into_iter()
                .map(|response| response.data)
                .collect(),
            id_to_subjects: hashy,
        })
    }
}

#[derive(Debug, Clone)]
pub struct SubjectWithType {
    subject: Subject,
    subject_type: SubjectType,
}

impl SubjectWithType {
    pub fn new(subject: Subject, subject_type: SubjectType) -> Self {
        SubjectWithType {
            subject,
            subject_type,
        }
    }
}
