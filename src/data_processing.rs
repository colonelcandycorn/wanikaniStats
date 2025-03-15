pub mod api_client;
pub mod complete_user_info;

use chrono::{DateTime, Local};
use governor::DefaultDirectRateLimiter;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error;
use std::fmt;
use std::marker::PhantomData;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
struct User {
    level: i32,
    username: String,
    started_at: DateTime<Local>,
}

#[derive(Deserialize, Serialize, Debug)]
struct PageData {
    per_page: i32,
    next_url: Option<String>,
    previous_url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct ReviewStatistic {
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
struct Meanings {
    meaning: Option<String>,
    primary: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Subject {
    characters: Option<String>,
    level: i32,
    spaced_repetition_system_id: i32,
    meanings: Vec<Meanings>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Assignment {
    created_at: Option<DateTime<Local>>,
    passed_at: Option<DateTime<Local>>,
    srs_stage: i32,
    subject_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Reset {
    created_at: DateTime<Local>,
    confirmed_at: DateTime<Local>,
    original_level: i32,
    target_level: i32,
}

#[derive(Deserialize, Serialize, Debug)]
struct PagedData<T> {
    pages: Option<PageData>,
    total_count: i32,
    data: Vec<Response<T>>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Response<T> {
    id: Option<i32>,
    object: String,
    data: T,
}

struct ReqwestResponse<T> {
    raw_response: reqwest::Response,
    resource_type: PhantomData<T>,
}

/// This is the main struct that will be used to interact with the WaniKani API.
/// You really just need to create an instance of this struct and then call the
/// `build_complete_user_info` method. This will return a `CompleteUserInfo` struct that will
/// contain all the information that you need.
#[derive(Debug)]
pub struct ApiClient<'a> {
    token: String,
    client: &'a reqwest::Client,
    limiter: &'a DefaultDirectRateLimiter,
}

#[derive(Debug, Clone)]
struct CompleteUserInfoBuilder {
    user: User,
    review_stats: Vec<ReviewStatistic>,
    assignments: Vec<Assignment>,
    resets: Vec<Reset>,
    id_to_subjects: HashMap<i32, SubjectWithType>,
}

/// This is the most important struct in the project. As the entire purpose of this
/// project is really just to get this information and present it to the user. This will
/// contain all the information that was gathered from the API. This struct is also
/// responsible for calculating the stats for each type of subject. The main thing that
/// I am dissatisfied with is the fact that the `id_to_subjects` field  contains the actual
/// subject data. This is because this information is not specific to the user and could
/// be used for other users. Ideally, this would be a borrowed reference to the subject
/// data that would be stored somewhere else.
#[derive(Debug, Clone)]
#[allow(unused)]
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
enum SubjectType {
    KanaVocabulary,
    Kanji,
    Radical,
    Vocabulary,
}

/// This is a custom error type that would really only occur if the API response
/// was not what was expected. This is a very unlikely error to occur and if it
/// does, then that would be on WaniKani's end.
#[derive(Debug)]
pub struct MissingSubjectError;

impl fmt::Display for MissingSubjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Missing Subject")
    }
}

impl error::Error for MissingSubjectError {}

/// This is one of the only structs that has every field public. This is because
/// the fields are basically exactly what goes into the templates. This is reliant
/// on the enum `SubjectType` to determine what type of subject it is. So there will
/// be four of these structs in the `CompleteUserInfo` struct.
#[derive(Debug, Clone)]
#[allow(unused)]
struct SubjectTypeStats {
    pub subject_type: SubjectType,
    pub num_of_meaning_correct: i32,
    pub num_of_meaning_incorrect: i32,
    pub num_of_reading_correct: i32,
    pub num_of_reading_incorrect: i32,
}

#[derive(Debug, Clone)]
#[allow(unused)]
struct SubjectWithType {
    subject: Subject,
    subject_type: SubjectType,
}
