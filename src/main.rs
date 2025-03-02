use wanikani_stats::data_processing::api_client;
use wanikani_stats::data_processing::api_client::ApiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let wk_token = std::env::var("WK_TOKEN").expect("WK_TOKEN must be set");
    let test_user_url = "https://api.wanikani.com/v2/user";
    // let resets_url = "https://api.wanikani.com/v2/resets";
    // let review_stats_url = "https://api.wanikani.com/v2/review_statistics";
    // let subject_url = "https://api.wanikani.com/v2/subjects/440";
    // let assignments_url = "https://api.wanikani.com/v2/assignments";

    let client = reqwest::Client::new();

    let api_client = ApiClient::new(wk_token, &client);

    let raw = api_client
        .get_response::<api_client::Response<api_client::User>>(test_user_url)
        .await
        .unwrap();

    let processed = api_client.raw_response_to_data(raw).await.unwrap();

    println!("{:?}", processed);

    Ok(())
}
