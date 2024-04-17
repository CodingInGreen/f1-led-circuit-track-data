use reqwest::Error;
use serde_json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Replace `your_endpoint` with the actual endpoint you need to access
    let url = "https://api.openf1.org/v1/drivers?driver_number=1&session_key=9158";

    // Sending a GET request
    let response = reqwest::get(url).await?;

    // Checking if the request was successful
    if response.status().is_success() {
        // Parse the response body as JSON
        let result: serde_json::Value = response.json().await?;
        println!("Response: {:?}", result);
    } else {
        // Handle the case where the request failed
        println!("Failed to get data: {}", response.status());
    }

    Ok(())
}
