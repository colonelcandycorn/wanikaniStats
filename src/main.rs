use governor::{Quota, RateLimiter};
use nonzero_ext::*;
use serde::Deserialize;
use std::{collections::HashMap, sync::{Arc, RwLock}};
use wanikani_stats::data_processing::api_client::{ApiClient, CompleteUserInfo};
use moka::future::Cache;
use axum::{
    extract::State, http::StatusCode, response::{AppendHeaders, Html, IntoResponse, Redirect, Response}, Form, Router, routing::{get, post}
};
use uuid::Uuid;
use axum_extra::extract::cookie::{ Cookie, CookieJar };


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
}

impl AppState {
    async fn get_or_cache_user_data(&self, token: UserToken) -> CompleteUserInfo {
        todo!()
    }
}

#[axum::debug_handler]
async fn post_login(jar: CookieJar, State(state): State<AppState>, Form(wk_token_form): Form<TokenForm>) -> Result<(CookieJar, Redirect), StatusCode> {
    let user_uuid = Uuid::new_v4().to_string();
    let user_token = UserToken { token: wk_token_form.wk_token };
    state.cookie_to_token.write().unwrap().insert(user_uuid.clone(), user_token);
    Ok((jar.add(Cookie::new("user_uuid", user_uuid)), Redirect::to("/info")))
}

async fn get_login(jar: CookieJar) -> Response {
    if let Some(_) = jar.get("user_uuid") {
        return Redirect::to("/info").into_response()
    }

    Html("<!DOCTYPE html> \
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <title>WaniKani Login</title>
</head>
<body>
    <h1>Enter Your WaniKani API Token</h1>
    <form action=\"/login\" method=\"post\">
        <label for=\"api_token\">WaniKani API Token:</label>
        <input type=\"text\" id=\"api_token\" name=\"wk_token\" required>
        <button type=\"submit\">Submit</button>
    </form>
</body>
</html>").into_response()
}

async fn get_info(jar: CookieJar, State(state): State<AppState>) -> Response {
    if let Some(user_uuid) = jar.get("user_uuid") {
        let state = state.cookie_to_token.read().unwrap();
        let user_token = state.get(user_uuid.value());

        return Html(format!("
        <p>uuid: {:?} token: {:?}</p>
        ", user_uuid.value(), user_token.unwrap())).into_response()
    }

    StatusCode::UNAUTHORIZED.into_response()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let shared_state = AppState {
        cookie_to_token: Arc::new(RwLock::new(HashMap::new())),
        user_info_cache: Cache::new(1000),
        rate_limiter: Arc::new(RateLimiter::direct_with_clock(
            Quota::per_minute(nonzero!(10u32)),
            governor::clock::DefaultClock::default())),
        reqwest_client: reqwest::Client::new()
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
