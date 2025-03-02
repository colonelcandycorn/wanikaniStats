use wanikani_stats::data_processing::api_client::ApiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let wk_token = std::env::var("WK_TOKEN").expect("WK_TOKEN must be set");

    let client = reqwest::Client::new();

    let api_client = ApiClient::new(wk_token, &client);

    let user_data = api_client.get_user_data().await?;
    let review_data = api_client.get_all_review_statistics().await?;

    println!("{:?}", user_data);
    println!("{:?}", &review_data[..3]);

    Ok(())
}
