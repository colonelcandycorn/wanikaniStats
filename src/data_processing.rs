pub mod api_client;
pub mod complete_user_info;

use chrono::{DateTime, Local};
use governor::DefaultDirectRateLimiter;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error;
use std::fmt;
use std::marker::PhantomData;



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

#[derive(Debug, Clone)]
pub struct CompleteUserInfoBuilder {
    user: User,
    review_stats: Vec<ReviewStatistic>,
    assignments: Vec<Assignment>,
    resets: Vec<Reset>,
    id_to_subjects: HashMap<i32, SubjectWithType>,
}

#[derive(Debug, Clone)]
pub struct CompleteUserInfo {
    user: User,
    review_stats: Vec<ReviewStatistic>,
    assignments: Vec<Assignment>,
    resets: Vec<Reset>,
    id_to_subjects: HashMap<i32, SubjectWithType>,
    kanji_learned: i32,
    radicals_learned: i32,
    vocab_learned: i32,
    kana_learned: i32,
    kanji_stats: SubjectTypeStats,
    radical_stats: SubjectTypeStats,
    vocab_stats: SubjectTypeStats,
    kana_stats: SubjectTypeStats,
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

#[derive(Debug, Clone)]
pub struct SubjectTypeStats {
    pub subject_type: SubjectType,
    pub num_of_meaning_correct: i32,
    pub num_of_meaning_incorrect: i32,
    pub num_of_reading_correct: i32,
    pub num_of_reading_incorrect: i32,
}

#[derive(Debug, Clone)]
pub struct SubjectWithType {
    subject: Subject,
    subject_type: SubjectType,
}