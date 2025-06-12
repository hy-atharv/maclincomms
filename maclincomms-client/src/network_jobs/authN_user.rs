use reqwest::{Client, Response, Error};



pub async fn authenticate_user(
    token: String, 
    authN_endpoint: &'static str
) -> Result<Response, Error> {
    
   
    let url = authN_endpoint.to_string();
    let client = Client::new();

    let response = client
        .get(url)
        .header("Authorization", token)
        .send()
        .await?;

    return Ok(response);

}