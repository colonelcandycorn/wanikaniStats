use governor::{Quota, RateLimiter};
use nonzero_ext::*;
use wanikani_stats::data_processing::api_client::{ApiClient, ReviewStatistic};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let wk_token = std::env::var("WK_TOKEN").expect("WK_TOKEN must be set");

    let client = reqwest::Client::new();
    let limiter = Arc::new(RateLimiter::direct_with_clock(
        Quota::per_minute(nonzero!(10u32)),
        governor::clock::DefaultClock::default(),
    ));

    let api_client = ApiClient::new(wk_token, &client, &limiter);

    let complete_user_information = api_client.build_complete_user_info().await?;

    complete_user_information.pretty_print();

    Ok(())
}
