use wanikani_stats::data_processing::api_client::{ApiClient, ReviewStatistic};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let wk_token = std::env::var("WK_TOKEN").expect("WK_TOKEN must be set");

    let client = reqwest::Client::new();

    let api_client = ApiClient::new(wk_token, &client);

    let user_data = api_client.get_user_data().await?;
    let review_data = api_client.get_all_pages_of_paged_data::<ReviewStatistic>("https://api.wanikani.com/v2/review_statistics").await?;
    // let sub_vec = api_client.get_list_of_subjects_to_request(&review_data);
    // let hashy = api_client.construct_id_to_subject_hash(&sub_vec).await?;

    // println!("{:?}", hashy.keys());
    println!("{:?}", user_data);
    println!("{:?}", &review_data[..3]);

    Ok(())
}
