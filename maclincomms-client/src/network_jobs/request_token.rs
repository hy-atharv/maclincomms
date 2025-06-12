use reqwest::{Client, Response, Error};
use tokio::runtime::Runtime;

use crate::user_model::RequestNewTokenResponse;



pub async fn request_new_token(
    refresh_token: String, 
    new_token_endpoint: &'static str
) -> Result<RequestNewTokenResponse, Error> {
    
   
    let url = new_token_endpoint.to_string();
    let client = Client::new();

    let response = client
        .get(url)
        .header("Authorization", refresh_token)
        .send()
        .await?;

    let data = response.json::<RequestNewTokenResponse>().await?;

    return Ok(data);

}