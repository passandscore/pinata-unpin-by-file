use std::env;
use dotenv::dotenv;
use pinata_sdk::PinataApi;

#[tokio::main]
pub async fn main_with_args(hash: &str, index: usize) -> Result<(), Box<dyn std::error::Error>> {
    // Load the environment variables from the .env file
    dotenv().ok();

    // Read the API key and secret from environment variables
    let api_key = env::var("PINATA_API_KEY")?;
    let api_secret = env::var("PINATA_API_SECRET")?;

    // Create an instance of PinataApi
    let api = PinataApi::new(&api_key, &api_secret)?;

    // Unpin the content
    let unpin_result = api.unpin(hash).await;

    // Return the result
    match unpin_result {
        Ok(_) => {
            println!("{}) Successfully unpinned", index);
            println!();
        }
        Err(_) => {
            println!();
            println!("{}) CID not found: {}", index, hash);
        }
    }

    Ok(())
}
