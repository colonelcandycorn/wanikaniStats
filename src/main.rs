use axum::{
    extract::State,
    http::StatusCode,
    response::{AppendHeaders, Html, IntoResponse, Redirect, Response},
    routing::{get, post},
    Form, Router,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use governor::{Quota, RateLimiter};
use minijinja::{context, Environment};
use moka::{future::Cache, Expiry};
use nonzero_ext::*;
use serde::Deserialize;
use std::time::{Duration, Instant};
use std::{collections::HashMap, ops::Sub, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;
use wanikani_stats::data_processing::{ApiClient, CompleteUserInfo, SubjectType, SubjectTypeStats};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct UserToken {
    token: String,
}

#[derive(Deserialize)]
struct TokenForm {
    wk_token: String,
}

#[derive(Clone)]
struct AppState {
    cookie_to_token: Arc<RwLock<HashMap<String, UserToken>>>,
    user_info_cache: Cache<UserToken, CompleteUserInfo>,
    rate_limiter: Arc<governor::DefaultDirectRateLimiter>,
    reqwest_client: reqwest::Client,
    env: Environment<'static>,
}

impl AppState {
    async fn get_or_cache_user_data(&self, token: &UserToken) -> Option<CompleteUserInfo> {
        let rate_limiter = self.rate_limiter.clone();
        let user_info_cache = self.user_info_cache.clone();
        let reqwest_client = self.reqwest_client.clone();
        let token = token.clone();
        let user_info = user_info_cache
            .get_with(token.clone(), async move {
                let rate_limiter = rate_limiter.clone();
                let reqwest_client = reqwest_client.clone();
                let token = token.clone();
                let api_client = ApiClient::new(token.token, &reqwest_client, &rate_limiter);
                let user_info = api_client.build_complete_user_info().await.unwrap();
                user_info
            })
            .await;

        Some(user_info)
    }
}

#[axum::debug_handler]
async fn post_login(
    jar: CookieJar,
    State(state): State<AppState>,
    Form(wk_token_form): Form<TokenForm>,
) -> Result<(CookieJar, Redirect), StatusCode> {
    let user_uuid = Uuid::new_v4().to_string();
    let user_token = UserToken {
        token: wk_token_form.wk_token,
    };
    state
        .cookie_to_token
        .write()
        .await
        .insert(user_uuid.clone(), user_token);
    Ok((
        jar.add(Cookie::new("user_uuid", user_uuid)),
        Redirect::to("/info"),
    ))
}

async fn get_login(jar: CookieJar, State(state): State<AppState>) -> Response {
    if let Some(_) = jar.get("user_uuid") {
        return Redirect::to("/info").into_response();
    }

    let template = state.env.get_template("login").unwrap();

    let rendered = template.render(context! {}).unwrap();

    Html(rendered).into_response()
}

#[axum::debug_handler]
async fn get_info(jar: CookieJar, State(state): State<AppState>) -> Response {
    if let Some(user_uuid) = jar.get("user_uuid") {
        let user_token_state = state.cookie_to_token.read().await;
        let user_token = user_token_state.get(user_uuid.value());

        if let Some(user_token) = user_token {
            if let Some(user_info) = state.get_or_cache_user_data(user_token).await {
                // If we made ALL the way here, we have a user info to display
                let template = state.env.get_template("info").unwrap();

                let started_date = user_info.get_started_at();
                let current_date = chrono::Local::now();
                let days_since_start = (current_date - started_date).num_days();
                let reset_count = user_info.get_num_of_resets();
                let reset_date = user_info
                    .get_date_of_most_recent_reset()
                    .unwrap_or(started_date);
                let days_since_reset = (current_date - reset_date).num_days();

                let context = context! {
                    username => user_info.get_user_name(),
                    level => user_info.get_level(),
                    started_date => started_date,
                    start_day_count => days_since_start,
                    reset_count => reset_count,
                    reset_date => reset_date,
                    reset_day_count => days_since_reset,
                    kanji_learned => user_info.get_kanji_learned(),
                    radicals_learned => user_info.get_radicals_learned(),
                    vocab_learned => user_info.get_vocab_learned(),
                    total_reading_count => user_info.get_total_reading_count(),
                    total_meaning_count => user_info.get_total_meaning_count(),
                    total_review_count => user_info.get_total_count(),
                    total_correct_count => user_info.get_total_correct_count(),
                    total_correct_reading_count => user_info.get_total_correct_reading_count(),
                    total_correct_meaning_count => user_info.get_total_correct_meaning_count(),
                    total_incorrect_count => user_info.get_total_incorrect_count(),
                    total_incorrect_reading_count => user_info.get_total_incorrect_reading_count(),
                    total_incorrect_meaning_count => user_info.get_total_incorrect_meaning_count(),
                    total_accuracy => format!("{:.2}",user_info.get_total_accuracy()),
                    reading_accuracy => format!("{:.2}",user_info.get_total_reading_accuracy()),
                    meaning_accuracy => format!("{:.2}",user_info.get_total_meaning_accuracy()),
                    radical_meaning_accuracy => format!("{:.2}",user_info.get_radical_meaning_accuracy()),
                    kanji_reading_accuracy => format!("{:.2}",user_info.get_kanji_reading_accuracy()),
                    kanji_meaning_accuracy => format!("{:.2}",user_info.get_kanji_meaning_accuracy()),
                    kanji_total_accuracy => format!("{:.2}",user_info.get_kanji_total_accuracy()),
                    vocab_reading_accuracy => format!("{:.2}",user_info.get_vocab_reading_accuracy()),
                    vocab_meaning_accuracy => format!("{:.2}",user_info.get_vocab_meaning_accuracy()),
                    vocab_total_accuracy => format!("{:.2}",user_info.get_vocab_total_accuracy()),
                };

                let rendered = template.render(context).unwrap();

                return Html(rendered).into_response();
            }

            // if we get to this point, the user has a cookie with an uuid and that uuid has an associated token
            // but the token was unable to be used to get user info which means wanikani has no account associated with that token
            // so we remove the cookie and redirect to login
            let jar = jar.remove(Cookie::from("user_uuid"));

            let result: Result<(CookieJar, Redirect), StatusCode> = Ok((
                jar.add(Cookie::new(
                    "flash",
                    "No account associated with that Token",
                )),
                Redirect::to("/login"),
            ));

            return result.into_response();
        }

        // if we get to this point, the user has a cookie with an uuid but that uuid has no associated token
        // so we remove the cookie and redirect to login
        let result: Result<(CookieJar, Redirect), StatusCode> = Ok((
            jar.remove(Cookie::from("user_uuid")),
            Redirect::to("/login"),
        ));

        return result.into_response();
    }

    // if we get to this point, the user has no cookie so has never logged in and should be redirected to login
    let result: Result<(CookieJar, Redirect), StatusCode> = Ok((jar, Redirect::to("/login")));

    result.into_response()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut env = Environment::new();

    env.add_template("base", include_str!("../templates/base.jinja"))
        .unwrap();

    env.add_template("login", include_str!("../templates/login.jinja"))
        .unwrap();

    env.add_template("info", include_str!("../templates/info.jinja"))
        .unwrap();

    let cache = Cache::builder()
        .max_capacity(1000)
        .time_to_live(Duration::from_secs(300))
        .build();

    let shared_state = AppState {
        cookie_to_token: Arc::new(RwLock::new(HashMap::new())),
        user_info_cache: cache,
        rate_limiter: Arc::new(RateLimiter::direct_with_clock(
            Quota::per_minute(nonzero!(10u32)),
            governor::clock::DefaultClock::default(),
        )),
        reqwest_client: reqwest::Client::new(),
        env: env,
    };

    let app = Router::new()
        .route("/", get(get_login))
        .route("/login", get(get_login).post(post_login))
        .route("/info", get(get_info))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
